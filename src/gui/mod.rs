use std::time::SystemTime;

use iced::{Container, Element, Length, Row, Sandbox, Scrollable, Space, TextInput, button, scrollable};
use iced::{Button, Column, Text};
use rustcord::{EventHandlers, RichPresenceBuilder, Rustcord, User};

struct Handlers;

impl EventHandlers for Handlers {
}

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
                Text::new(self.value.to_string()).size(50).width(Length::FillPortion(5)).horizontal_alignment(iced::HorizontalAlignment::Center),
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
    text_box: iced::text_input::State
}

impl Sandbox for Tour {
    type Message = Message;

    fn new() -> Self {
        let discord = Rustcord::init::<Handlers>("957307903931465738", true, None)
            .expect("Could no initialize RPC");

        Self {
            page: 1,
            book_name: String::new(),
            start_time: SystemTime::now(),
            discord,

            counter: Counter::new(1),
            text_box: iced::text_input::State::new()
        }
    }

    fn title(&self) -> String {
        format!("本を読む")
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::IncrementPressed => {
                self.page += 1;
            },
            Message::DecrementPressed => {
                self.page -= 1;
            },
            Message::EditName(name) => {
                self.book_name = name
            }
        }
        self.counter.update(self.page);

        let presence = RichPresenceBuilder::new()
            .state(&format!("p.{}", self.page))
            .details(&self.book_name)
            .start_time(self.start_time)
            .build();

        self.discord.update_presence(presence).expect("Could not update presence");
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
        let text_box = TextInput::new(text_box, "Enter book name", book_name, |e| Message::EditName(e));

        controls = controls.push(text_box);
        controls = controls.push(Space::with_width(Length::Fill));
        controls = controls.push(counter.view());

        let content: Element<_> = Column::new()
            .max_width(540)
            .spacing(20)
            .padding(20)
            .push(controls)
            .into();

        self.discord.run_callbacks();

        Container::new(content).width(Length::Fill).center_x()
            .height(Length::Fill)
            .center_y()
            .into()
            
    }
}
