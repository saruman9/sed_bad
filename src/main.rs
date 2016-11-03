extern crate chrono;
extern crate gtk;

mod document;
mod metadata;
mod user;
mod category;
mod comment;
mod permission;
mod ui;

use ui::MainUI;

fn main() {
    let ui = MainUI::init();
    ui.borrow().run();
}
