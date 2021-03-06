use std::time::SystemTime;

use iced::keyboard;
use iced::{
    button, Application, Clipboard, Command, Container, Element, Length, Row, Space,
    TextInput,
};
use iced::{Button, Column, Text};
use rustcord::{EventHandlers, RichPresenceBuilder, Rustcord};

use crate::save::{save_settings, AppState};

struct Handlers;

impl EventHandlers for Handlers {}

struct Counter {
    // The counter value
    value: i32,
    text_box: iced::text_input::State,

    // The local state of the two buttons
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    SetPage(i32),
    EditName(String),
    None,
}

impl Counter {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
            text_box: iced::text_input::State::new(),
        }
    }

    pub fn view(&mut self) -> Row<Message> {
        let text_box = TextInput::new(
            &mut self.text_box,
            "Enter page",
            &self.value.to_string(),
            |e| {
                if e.is_empty() {
                    return Message::SetPage(0);
                }
                let p = e.parse();
                match p {
                    Ok(p) => Message::SetPage(p),
                    Err(_e) => Message::None,
                }
            },
        )
        .size(50)
        .width(Length::FillPortion(5));

        Row::new()
            .push(
                Button::new(&mut self.decrement_button, Text::new("-"))
                    .on_press(Message::DecrementPressed),
            )
            .push(text_box)
            .push(
                Button::new(&mut self.increment_button, Text::new("+"))
                    .on_press(Message::IncrementPressed),
            )
    }

    pub fn update(&mut self, message: i32) {
        self.value = message;
    }
}

pub struct Tour {
    page: i32,
    book_name: String,
    start_time: SystemTime,
    discord: Rustcord,

    counter: Counter,
    text_box: iced::text_input::State,
}

impl Tour {
    fn update_presence(&self) {
        let presence = RichPresenceBuilder::new()
            .state(&format!("p.{}", self.page))
            .details(&self.book_name)
            .start_time(self.start_time)
            .build();

        self.discord
            .update_presence(presence)
            .expect("Could not update presence");
    }
}

fn handle_key(key_code: keyboard::KeyCode) -> Option<Message> {
    match key_code {
        keyboard::KeyCode::Left => Some(Message::DecrementPressed),
        keyboard::KeyCode::Right => Some(Message::IncrementPressed),
        keyboard::KeyCode::Space => Some(Message::IncrementPressed),
        keyboard::KeyCode::Enter => Some(Message::IncrementPressed),
        _ => None,
    }
}

impl Application for Tour {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Flags = AppState;

    fn new(app_state: Self::Flags) -> (Self, Command<Message>) {
        let discord = Rustcord::init::<Handlers>("957307903931465738", true, None)
            .expect("Could no initialize RPC");

        let res = Self {
            page: app_state.page,
            book_name: app_state.book_name,
            start_time: SystemTime::now(),
            discord,

            counter: Counter::new(app_state.page),
            text_box: iced::text_input::State::new(),
        };
        res.update_presence();
        (res, Command::none())
    }

    fn title(&self) -> String {
        "????????????".to_string()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        use iced_native::{keyboard, subscription, Event};

        subscription::events_with(|event, status| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                modifiers: _,
                key_code,
            }) => {
                if status == iced_native::event::Status::Ignored {
                    handle_key(key_code)
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    fn update(&mut self, event: Message, _: &mut Clipboard) -> Command<Message> {
        match event {
            Message::IncrementPressed => {
                self.page += 1;
            }
            Message::DecrementPressed => {
                self.page -= 1;
            }
            Message::EditName(name) => self.book_name = name,
            Message::SetPage(p) => self.page = p,
            Message::None => (),
        }
        self.counter.update(self.page);
        save_settings(AppState {
            book_name: self.book_name.clone(),
            page: self.page,
        });
        self.update_presence();

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let Tour {
            page: _,
            book_name,

            counter,
            text_box,
            ..
        } = self;

        let mut controls = Column::new();
        let text_box = TextInput::new(text_box, "Enter book name", book_name, |e| {
            Message::EditName(e)
        })
        .size(50);

        controls = controls.push(text_box);
        controls = controls.push(Space::with_height(Length::Units(50)));
        controls = controls.push(counter.view());

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(controls)
            .into();

        self.discord.run_callbacks();

        Container::new(content)
            .width(Length::Fill)
            .center_x()
            .height(Length::Fill)
            .center_y()
            .into()
    }
}
