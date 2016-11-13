//! Dialog of creating new ticket.
//!
//! TODO Write docs.

use gtk;

use super::MainUI;
use super::utils::show_error_dialog;
use category::Category;
use user::User;

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
        self.connect_signals_cancel_button();
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
