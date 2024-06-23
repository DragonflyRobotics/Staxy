use gtk::{Application, ApplicationWindow};
use gtk::prelude::*;


pub fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    // Present window
    window.present();
}
