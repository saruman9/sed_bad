//! Auxiliary functions for user interface.
//!
//! TODO Write docs.

use gtk;

pub fn show_error_dialog<W: gtk::IsA<gtk::Window>>(parent: &W, message: &str) {
    use gtk::{DialogExt, WidgetExt};

    let error_dialog = gtk::MessageDialog::new(Some(parent),
                                               gtk::DIALOG_MODAL,
                                               gtk::MessageType::Error,
                                               gtk::ButtonsType::Ok,
                                               message);
    error_dialog.run();
    error_dialog.destroy();
}
