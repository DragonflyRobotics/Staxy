mod clipboard_manager;
mod ui;

use std::cell::RefCell;
use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};

use arboard::Clipboard;

use rdev::listen;

use gtk::prelude::*;
use gtk::{glib, Application};

const APP_ID: &str = "org.gtk_rs.HelloWorld1";

fn main() -> glib::ExitCode {

    // let (sender, receiver): (Sender<Vec<String>>, Receiver<Vec<String>>) = channel();
    // let clipboard_thread = thread::spawn(move || {
    //     let mut stack: Vec<String> = Vec::new();
    //     let mut clipboard = Clipboard::new().unwrap();
    //     let mut ctrl_pressed = false;
    //     if let Err(error) = listen(move |event| clipboard_manager::callback(event, &mut ctrl_pressed, &mut stack, &mut clipboard, sender.clone())) {
    //         println!("Error: {:?}", error)
    //     }
    // });
    // 
    // loop {
    //     let stack = receiver.recv().unwrap();
    //     println!("Stack: {:?}", stack);
    // }
    
    // let _ = clipboard_thread.join();
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(ui::build_ui);
    app.run()
}

struct StaxyFlags {
    receiver: Receiver<Vec<String>>
}

struct Staxy {
    receiver: RefCell<Option<Receiver<Vec<String>>>>,
    stack: Vec<String>
}


