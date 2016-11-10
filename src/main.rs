extern crate chrono;
extern crate md5;
extern crate gtk;

mod document;
mod metadata;
mod user;
mod category;
mod comment;
mod permission;
mod ui;
mod errors;
mod db;

use ui::MainUI;

fn main() {
    let ui = MainUI::init();
    ui.run();
}
