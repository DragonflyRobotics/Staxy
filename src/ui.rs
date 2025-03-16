
use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_ui(app: &Application, receiver: Arc<Mutex<Receiver<Vec<String>>>>) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Create a label
    let label = Label::new(None);
    window.set_child(Some(&label));

    let label_rc = Rc::new(RefCell::new(label));

    // Add an idle source to the default main loop context
    glib::timeout_add_local(std::time::Duration::from_millis(100), move || {
        let receiver_clone = Arc::clone(&receiver);
        // Lock the receiver to check for updates
        if let Ok(receiver) = receiver_clone.lock() {
            // Check if there are any updates from the receiver
            if let Ok(stack) = receiver.try_recv() {
                // Update the label with the new stack
                label_rc.borrow().set_text(&format!("Stack: {:?}", stack));
            }
        }
        // Continue calling this function
        glib::Continue(true)
    });

    // Present window
    window.present();
}

