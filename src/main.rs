use iced::{Sandbox, Settings};
use iced::widget::{button, Button, column, Column, text, Text};

#[derive(Default)]
struct MyApp {
    button_state: button::State,
    counter: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ButtonPressed,
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        MyApp::default()
    }

    fn title(&self) -> String {
        String::from("Minimal Iced Beispiel 0.12")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => self.counter += 1,
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let btn = Button::new(Text::new("Klick mich!"))
                .on_press(Message::ButtonPressed);

        let content = Column::new()
            .push(text(format!("Counter: {}", self.counter)))
            .push(btn);

        content.into()
    }
}

fn main() -> iced::Result {
    MyApp::run(Settings::default())
}
