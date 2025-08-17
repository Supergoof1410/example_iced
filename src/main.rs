use iced::{Sandbox, Settings};
use iced::widget::{button, Button, column, Column, text, Text};

#[derive(Default)]
struct MyApp {
    counter: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ChangeCounter(i32),   // eine Message mit Parameter
}

impl Sandbox for MyApp {
    type Message = Message;

    fn new() -> Self {
        MyApp::default()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn title(&self) -> String {
        String::from("Counter mit Parametern")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeCounter(delta) => self.counter += delta,
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        // Button "Hoch" gibt +1 weiter
        let btn_up: Button<'_, Message> = Button::new(Text::new("Hoch"))
            .on_press(Message::ChangeCounter(1));

        // Button "Runter" gibt -1 weiter
        let btn_down: Button<'_, Message> = Button::new(Text::new("Runter"))
            .on_press(Message::ChangeCounter(-1));

        // Alles zusammen in einer Spalte
        Column::new()
            .push(text(format!("Counter: {}", self.counter)))
            .push(btn_up)
            .push(btn_down)
            .into()
    }
}

fn main() {
    MyApp::run(Settings::default());
}
