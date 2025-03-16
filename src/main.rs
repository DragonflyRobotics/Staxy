mod clipboard_manager;
mod ui;

use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

use arboard::Clipboard;
use gtk::prelude::*;
use gtk::{glib, Application};
use rdev::listen;

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {
    let (sender, receiver): (Sender<Vec<String>>, Receiver<Vec<String>>) = channel();
    let clipboard_thread = thread::spawn(move || {
        let mut stack: Vec<String> = Vec::new();
        let mut clipboard = Clipboard::new().unwrap();
        let mut ctrl_pressed = false;
        if let Err(error) = listen(move |event| clipboard_manager::callback(event, &mut ctrl_pressed, &mut stack, &mut clipboard, sender.clone())) {
            println!("Error: {:?}", error);
        }
    });

    let receiver = Arc::new(Mutex::new(receiver));
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(move |app| {
        ui::build_ui(app, Arc::clone(&receiver));
    });
    app.run()
}

