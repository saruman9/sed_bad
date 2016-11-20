//! Dialog of creating new ticket.
//!
//! TODO Write docs.

use gtk;
use rusqlite;

use super::MainUI;
use super::utils::show_error_dialog;
use category::Category;
use user::User;
use comment::Comment;
use document::Document;
use errors::DbError;

#[derive(Clone)]
pub struct NewTicket {
    main_ui: MainUI,
    dialog: gtk::Dialog,

    name_entry: gtk::Entry,
    category_label: gtk::Label,
    category_combobox: gtk::ComboBoxText,
    expired_label: gtk::Label,
    expired_calendar: gtk::Calendar,
    responsible_label: gtk::Label,
    responsible_combobox: gtk::ComboBoxText,
    commentary_label: gtk::Label,
    commentary_scrolled_window: gtk::ScrolledWindow,
    commentary_text: gtk::TextView,

    button_box: gtk::ButtonBox,
    ok_button: gtk::Button,
    cancel_button: gtk::Button,
}

impl NewTicket {
    pub fn new(main_ui: MainUI) -> Self {
        let tmp = NewTicket {
            main_ui: main_ui,
            dialog: gtk::Dialog::new(),

            name_entry: gtk::Entry::new(),
            category_label: gtk::Label::new(Some("Category:")),
            category_combobox: gtk::ComboBoxText::new_with_entry(),
            expired_label: gtk::Label::new(Some("Expired date:")),
            expired_calendar: gtk::Calendar::new(),
            responsible_label: gtk::Label::new(Some("Responsible user:")),
            responsible_combobox: gtk::ComboBoxText::new(),
            commentary_label: gtk::Label::new(Some("Commentary:")),
            commentary_scrolled_window: gtk::ScrolledWindow::new(None, None),
            commentary_text: gtk::TextView::new(),

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
        self.setup_name_entry();
        self.setup_expired_label();
        self.setup_commentary_label();
        self.setup_button_box();
        self.setup_category_label();
        self.setup_category_combobox();
        self.setup_responsible_label();
        self.setup_responsible_combobox();
        self.setup_dialog();
    }

    fn setup_name_entry(&self) {
        use gtk::EntryExt;

        self.name_entry.set_placeholder_text(Some("Name of ticket..."));
    }

    fn setup_expired_label(&self) {
        use gtk::WidgetExt;

        self.expired_label.set_halign(gtk::Align::Start);
    }

    fn setup_commentary_label(&self) {
        use gtk::WidgetExt;

        self.commentary_label.set_halign(gtk::Align::Start);
    }

    fn setup_button_box(&self) {
        self.button_box.set_layout(gtk::ButtonBoxStyle::Spread);
    }

    fn setup_category_label(&self) {
        use gtk::WidgetExt;

        self.category_label.set_halign(gtk::Align::Start);
    }

    fn setup_category_combobox(&self) {
        match Category::get_categories(&self.main_ui.db.borrow()) {
            Ok(categories) => {
                for category in categories {
                    self.category_combobox.append_text(category.name());
                }
            }
            Err(e) => {
                show_error_dialog(&self.dialog,
                                  &format!("Error of reading categories from database.\n{}", e));
            }
        }
    }

    fn setup_responsible_label(&self) {
        use gtk::WidgetExt;

        self.responsible_label.set_halign(gtk::Align::Start);
    }

    fn setup_responsible_combobox(&self) {
        match User::get_users(&self.main_ui.db.borrow()) {
            Ok(users) => {
                for user in users {
                    self.responsible_combobox.append_text(user.name());
                }
            }
            Err(e) => {
                show_error_dialog(&self.dialog,
                                  &format!("Error of reading users from database.\n{}", e));
            }
        }
    }

    fn setup_dialog(&self) {
        use gtk::WindowExt;

        self.dialog.set_title("New ticket");
        self.dialog.set_transient_for(Some(&self.main_ui.window));
        self.dialog.set_modal(true);

        self.dialog.set_default_size(500, 600);
    }

    fn connect_signals(&self) {
        self.connect_signals_ok_button();
        self.connect_signals_cancel_button();
    }

    fn connect_signals_ok_button(&self) {
        use gtk::{ButtonExt, EntryExt};

        let rc: NewTicket = self.clone();
        self.ok_button.connect_clicked(move |_| {
            // TODO Delete unwrap!
            let ticket_name = rc.name_entry.get_text().unwrap();
            if ticket_name.is_empty() {
                show_error_dialog(&rc.dialog, "Name of the ticket is empty.");
                return;
            }
            let category: Category;
            let category_str = rc.category_combobox.get_active_text().unwrap();
            if category_str.is_empty() {
                show_error_dialog(&rc.dialog, "Category is empty or not selected.");
                return;
            }
            match Category::get_category(&rc.main_ui.db.borrow(), &category_str) {
                Ok(category_row) => {
                    category = category_row;
                }
                Err(DbError::SqliteError(e)) => {
                    match e {
                        rusqlite::Error::QueryReturnedNoRows => {
                            let mut new_category: Category = Category::new(category_str);
                            match new_category.save_to_db(&rc.main_ui.db.borrow()) {
                                Ok(_) => {
                                    category = new_category;
                                }
                                Err(e) => {
                                    show_error_dialog(&rc.dialog,
                                                      &format!("Error of creating new category \
                                                                in database.\n{}",
                                                               e));
                                    return;
                                }
                            }
                        }
                        _ => {
                            show_error_dialog(&rc.dialog,
                                              &format!("Error of searching category in \
                                                        database.\n{}",
                                                       e));
                            return;
                        }
                    }
                }
                Err(e) => {
                    show_error_dialog(&rc.dialog,
                                      &format!("Error of searching category in \
                                                database.\n{}",
                                               e));
                    return;
                }
            }
            let expired_date = rc.expired_calendar.get_date();
            let responsible_user: User;
            if let Some(responsible_user_str) = rc.responsible_combobox.get_active_text() {
                match User::get_user(&rc.main_ui.db.borrow(), &responsible_user_str) {
                    Ok(res) => {
                        responsible_user = res;
                    }
                    Err(e) => {
                        show_error_dialog(&rc.dialog,
                                          &format!("Error of searching user in database.\n{}", e));
                        return;
                    }
                }
            } else {
                show_error_dialog(&rc.dialog, "Responsible user not selected.");
                return;
            }

            let commentary_buffer = rc.commentary_text.get_buffer().unwrap();
            let start_iter = commentary_buffer.get_start_iter();
            let end_iter = commentary_buffer.get_end_iter();
            let commentary_str = commentary_buffer.get_text(&start_iter, &end_iter, false).unwrap();
            let comment: Option<Comment>;
            if commentary_str.is_empty() {
                comment = None;
            } else {
                comment = Some(Comment::new(&rc.main_ui.current_user.borrow(), commentary_str));
            }

            let document = Document::new(ticket_name,
                                         &rc.main_ui.current_user.borrow(),
                                         category,
                                         responsible_user,
                                         expired_date,
                                         comment);
            println!("{:?}", document);
        });
    }

    fn connect_signals_cancel_button(&self) {
        use gtk::{ButtonExt, WidgetExt};

        let rc = self.clone();
        self.cancel_button.connect_clicked(move |_| {
            rc.dialog.destroy();
        });
    }

    fn pack_and_show(&self) {
        self.pack_button_box();
        self.pack_commentary_scrolled_window();
        self.pack_dialog();
    }

    fn pack_button_box(&self) {
        use gtk::ContainerExt;

        self.button_box.add(&self.ok_button);
        self.button_box.add(&self.cancel_button);
    }

    fn pack_commentary_scrolled_window(&self) {
        use gtk::ContainerExt;

        self.commentary_scrolled_window.add(&self.commentary_text);
    }

    fn pack_dialog(&self) {
        use gtk::{DialogExt, BoxExt, WidgetExt};

        let area = self.dialog.get_content_area();
        area.pack_start(&self.name_entry, false, false, 10);
        area.pack_start(&self.category_label, false, false, 10);
        area.pack_start(&self.category_combobox, false, false, 10);
        area.pack_start(&self.expired_label, false, false, 10);
        area.pack_start(&self.expired_calendar, false, false, 10);
        area.pack_start(&self.responsible_label, false, false, 10);
        area.pack_start(&self.responsible_combobox, false, false, 10);
        area.pack_start(&self.commentary_label, false, false, 10);
        area.pack_start(&self.commentary_scrolled_window, true, true, 10);
        area.pack_start(&self.button_box, false, false, 10);

        self.dialog.show_all();
    }
}
