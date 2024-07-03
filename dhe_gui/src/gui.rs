use iced::widget::{button, column, text};
use iced::{executor, Alignment, Application, Command, Element, Settings, Theme};

pub fn run() -> iced::Result {
    App::run(Settings::default())
}

struct App {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (App, Command<Self::Message>) {
        (App { value: 0 }, Command::none())
    }

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        column![
            button("Increment").on_press(Message::Increment),
            text(self.value).size(50),
            button("Decrement").on_press(Message::Decrement)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
