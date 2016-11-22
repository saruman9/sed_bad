//! Dialog of creating new comment.
//!
//! TODO Write docs.

use gtk;

use super::edit_ticket::EditTicket;
use super::utils::show_error_dialog;
use comment::Comment;

#[derive(Clone)]
pub struct NewComment {
    edit_ticket: EditTicket,
    dialog: gtk::Dialog,

    text_view: gtk::TextView,
    scrolled_window: gtk::ScrolledWindow,

    button_box: gtk::ButtonBox,
    ok_button: gtk::Button,
    cancel_button: gtk::Button,
}

impl NewComment {
    pub fn new(edit_ticket: EditTicket) -> Self {
        let tmp = NewComment {
            edit_ticket: edit_ticket.clone(),
            dialog: gtk::Dialog::new_with_buttons(Some("New comment"),
                                                  Some(&edit_ticket.dialog),
                                                  gtk::DIALOG_MODAL,
                                                  &[]),

            text_view: gtk::TextView::new(),
            scrolled_window: gtk::ScrolledWindow::new(None, None),

            button_box: gtk::ButtonBox::new(gtk::Orientation::Horizontal),
            ok_button: gtk::Button::new_with_mnemonic("_Ok"),
            cancel_button: gtk::Button::new_with_mnemonic("_Cancel"),
        };

        tmp.setup();
        tmp.connect_signals();
        tmp.pack_and_show();

        tmp
    }

    fn setup(&self) {
        self.setup_dialog();
    }

    fn setup_dialog(&self) {
        use gtk::WindowExt;

        self.dialog.set_default_size(500, 600);
    }

    fn connect_signals(&self) {
        self.ok_button_connect();
        self.cancel_button_connect();
    }

    fn ok_button_connect(&self) {
        use gtk::{ButtonExt, WidgetExt};

        let rc: NewComment = self.clone();
        self.ok_button.connect_clicked(move |_| {
            let buffer = rc.text_view.get_buffer().unwrap();
            let start_iter = buffer.get_start_iter();
            let end_iter = buffer.get_end_iter();
            let text = buffer.get_text(&start_iter, &end_iter, false).unwrap();
            let new_comment = Comment::new(&rc.edit_ticket.main_ui.current_user.borrow(), text);

            let res = rc.edit_ticket
                .doc
                .borrow_mut()
                .add_comment(&rc.edit_ticket.main_ui.db.borrow(), new_comment);
            match res {
                Ok(_) => {
                    rc.edit_ticket.update_main();
                    rc.dialog.destroy();
                }
                Err(e) => {
                    show_error_dialog(&rc.dialog,
                                      &format!("Error of creating new comment.\n{}", e));
                    return;
                }
            }
        });
    }

    fn cancel_button_connect(&self) {
        use gtk::{ButtonExt, WidgetExt};

        self.cancel_button.connect_clicked(|d| {
            d.destroy();
        });
    }

    fn pack_and_show(&self) {
        self.scrolled_window_pack();
        self.button_box_pack();
        self.dialog_pack();
    }

    fn scrolled_window_pack(&self) {
        use gtk::ContainerExt;

        self.scrolled_window.add(&self.text_view);
    }

    fn button_box_pack(&self) {
        use gtk::ContainerExt;

        self.button_box.add(&self.ok_button);
        self.button_box.add(&self.cancel_button);
    }

    fn dialog_pack(&self) {
        use gtk::{DialogExt, BoxExt, WidgetExt};

        let area = self.dialog.get_content_area();

        area.pack_start(&self.scrolled_window, true, true, 0);
        area.pack_start(&self.button_box, false, false, 0);

        self.dialog.show_all();
    }
}
