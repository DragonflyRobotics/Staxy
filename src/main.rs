mod clipboardManager;

use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

use arboard::Clipboard;

use rdev::{listen, Event, EventType, Key, simulate};

use iced::{Element, Sandbox, Settings};
use iced::widget::text;


fn main() {
    let (sender, receiver): (Sender<Vec<String>>, Receiver<Vec<String>>) = channel();
    let clipboardThread = thread::spawn(move || {
        let mut stack: Vec<String> = Vec::new();
        println!("Hello, world!");
        let mut clipboard = Clipboard::new().unwrap();

        let mut shiftPressed = false;
        let mut ctrlPressed = false;
        // sender.send(vec!["yo wassup".to_string()]).expect("uhh ohh");
        if let Err(error) = listen(move |event| clipboardManager::callback(event, &mut shiftPressed, &mut ctrlPressed, &mut stack, &mut clipboard, sender.clone())) {
            println!("Error: {:?}", error)
        }
    });

    println!("{:?}", receiver.recv().unwrap());
    Staxy::run(Settings::default());

    clipboardThread.join();

}

struct Staxy;

#[derive(Debug)]
enum Message {}

impl Sandbox for Staxy {
    type Message = Message;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("Stuff")
    }

    fn update(&mut self, message: Message) {
        match message {  }
    }

    fn view(&self) -> Element<'_, Message> {
        text("hello bozo").into()
    }


}