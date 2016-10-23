extern crate chrono;
extern crate gtk;

use gtk::{Window, WindowType, WindowExt, WidgetExt, ContainerExt, Inhibit, Button, ButtonExt};

mod document;
mod metadata;
mod user;
mod category;
mod comment;
mod permission;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("СЭД-БЭД");
    window.set_default_size(800, 600);

    let button = Button::new_with_label("Start");
    window.add(&button);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button.connect_clicked(|b| {
        b.set_label("Clicked!");
    });

    gtk::main();
}
