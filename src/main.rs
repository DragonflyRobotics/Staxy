mod clipboardManager;

use std::cell::RefCell;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::JoinHandle;

use arboard::Clipboard;

use rdev::{listen, Event, EventType, Key, simulate};

use iced::{Application, Command, Element, Error, Sandbox, Settings, Subscription};
use iced::keyboard::KeyCode::S;
use iced::widget::text;


fn main() {

    let (sender, receiver): (Sender<Vec<String>>, Receiver<Vec<String>>) = channel();
    let clipboardThread = thread::spawn(move || {
        let mut stack: Vec<String> = Vec::new();
        println!("Hello, world!");
        let mut clipboard = Clipboard::new().unwrap();

        let mut ctrlPressed = false;
        // sender.send(vec!["yo wassup".to_string()]).expect("uhh ohh");
        if let Err(error) = listen(move |event| clipboardManager::callback(event, &mut ctrlPressed, &mut stack, &mut clipboard, sender.clone())) {
            println!("Error: {:?}", error)
        }
    });

    Staxy::run(Settings::with_flags(StaxyFlags{receiver}));

    // let clipboardThread = Staxy::getClipboardThread();
    clipboardThread.join();

}

struct StaxyFlags {
    receiver: Receiver<Vec<String>>
}

struct Staxy {
    receiver: RefCell<Option<Receiver<Vec<String>>>>,
    stack: Vec<String>
}

#[derive(Debug)]
enum Message {
    ExternalMessageReceived(Vec<String>),
}

impl Application for Staxy {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = StaxyFlags;

    fn new(flags: StaxyFlags) -> (Self, Command<Message>) {
        let app = Staxy {
            receiver: RefCell::new(Some(flags.receiver)),
            stack: vec![],
        };

        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Stuff")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ExternalMessageReceived(num) => {
                self.stack = num;
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::subscription::unfold(
            "led changes",
            self.receiver.take(),
            move |mut receiver| async move {
                let num = receiver.as_mut().unwrap().recv().unwrap();
                (Message::ExternalMessageReceived(num), receiver)
            },
        )
    }

    fn view(&self) -> Element<'_, Message> {
        text(format!("{:?}", self.stack)).into()
    }



}

