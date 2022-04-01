use std::time::SystemTime;

use iced::keyboard::{self, Event};
use iced::{Application, Clipboard, Command, Container, Element, Length, Row, Sandbox, Scrollable, Space, TextInput, button, scrollable};
use iced::{Button, Column, Text};
use rustcord::{EventHandlers, RichPresenceBuilder, Rustcord, User};

use crate::save::{AppState, save_settings};

struct Handlers;

impl EventHandlers for Handlers {}

struct Counter {
    // The counter value
    value: i32,

    // The local state of the two buttons
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
    EditName(String),
}

impl Counter {
    pub fn new(value: i32) -> Self {
        Self {
            value,
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
        }
    }

    pub fn view(&mut self) -> Row<Message> {
        // We use a column: a simple vertical layout
        Row::new()
            .push(
                Button::new(&mut self.decrement_button, Text::new("-"))
                    .on_press(Message::DecrementPressed),
            )
            .push(
                // We show the value of the counter here
                Text::new(self.value.to_string())
                    .size(50)
                    .width(Length::FillPortion(5))
                    .horizontal_alignment(iced::HorizontalAlignment::Center),
            )
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
        format!("本を読む")
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
            },
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
            page,
            book_name,

            counter,
            text_box,
            ..
        } = self;

        let mut controls = Column::new();
        let text_box = TextInput::new(text_box, "Enter book name", book_name, |e| {
            Message::EditName(e)
        }).size(50);

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