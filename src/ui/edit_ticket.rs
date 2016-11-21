//! Dialog of editing/viewing information about ticket.
//!
//! TODO Write docs.

use gtk;
use chrono::{Datelike, UTC, TimeZone};

use std::rc::Rc;
use std::cell::RefCell;

use super::MainUI;
use document::Document;
use super::utils::show_error_dialog;
use user::User;
use category::Category;
use metadata::Status;
use comment::Comment;
use permission::{NaivePermission, Permission};

#[derive(Clone)]
pub struct EditTicket {
    pub main_ui: MainUI,
    pub doc: Rc<RefCell<Document>>,

    pub dialog: gtk::Dialog,

    pub name_entry: gtk::Entry,
    pub name_label: gtk::Label,
    pub c_time_label: gtk::Label,
    pub m_time_label: gtk::Label,
    pub author_label: gtk::Label,
    pub author_combobox: gtk::ComboBoxText,
    pub category_label: gtk::Label,
    pub category_combobox: gtk::ComboBoxText,
    pub status_label: gtk::Label,
    pub status_combobox: gtk::ComboBoxText,
    pub date_expired_label: gtk::Label,
    pub date_expired_calendar: gtk::Calendar,

    pub permission_author_label: gtk::Label,
    pub perm_author_button_box: gtk::ButtonBox,
    pub perm_author_read_but: gtk::ToggleButton,
    pub perm_author_write_but: gtk::ToggleButton,
    pub perm_author_comment_but: gtk::ToggleButton,
    pub permission_responsible_label: gtk::Label,
    pub perm_responsible_button_box: gtk::ButtonBox,
    pub perm_responsible_read_but: gtk::ToggleButton,
    pub perm_responsible_write_but: gtk::ToggleButton,
    pub perm_responsible_comment_but: gtk::ToggleButton,
    pub permission_others_label: gtk::Label,
    pub perm_others_button_box: gtk::ButtonBox,
    pub perm_others_read_but: gtk::ToggleButton,
    pub perm_others_write_but: gtk::ToggleButton,
    pub perm_others_comment_but: gtk::ToggleButton,

    pub data_entry: gtk::Entry,
    pub scrolled_window_label: gtk::Label,
    pub scrolled_window: gtk::ScrolledWindow,
    pub comments_text: gtk::TextView,
    pub responsible_label: gtk::Label,
    pub responsible_combobox: gtk::ComboBoxText,

    pub button_box: gtk::ButtonBox,
    pub ok_button: gtk::Button,
    pub cancel_button: gtk::Button,
}

impl EditTicket {
    pub fn new(main_ui: MainUI, document: Document) -> Self {
        let tmp = EditTicket {
            main_ui: main_ui,

            dialog: gtk::Dialog::new(),

            name_entry: gtk::Entry::new(),
            name_label: gtk::Label::new(Some("Name:")),
            c_time_label: gtk::Label::new(Some(&format!("Created at: {}",
                                                        document.metadata()
                                                            .c_time()
                                                            .to_rfc2822()))),
            m_time_label: gtk::Label::new(Some(&format!("Modified at: {}",
                                                        document.metadata()
                                                            .m_time()
                                                            .to_rfc2822()))),
            author_label: gtk::Label::new(Some("Author:")),
            author_combobox: gtk::ComboBoxText::new(),
            category_label: gtk::Label::new(Some("Category:")),
            category_combobox: gtk::ComboBoxText::new(),
            status_label: gtk::Label::new(Some("Status:")),
            status_combobox: gtk::ComboBoxText::new(),
            date_expired_label: gtk::Label::new(Some("Date expired:")),
            date_expired_calendar: gtk::Calendar::new(),

            permission_author_label: gtk::Label::new(Some("Author's permission:")),
            perm_author_button_box: gtk::ButtonBox::new(gtk::Orientation::Horizontal),
            perm_author_read_but: gtk::ToggleButton::new_with_label("Read"),
            perm_author_write_but: gtk::ToggleButton::new_with_label("Write"),
            perm_author_comment_but: gtk::ToggleButton::new_with_label("Comment"),
            permission_responsible_label: gtk::Label::new(Some("Responsible user's permission:")),
            perm_responsible_button_box: gtk::ButtonBox::new(gtk::Orientation::Horizontal),
            perm_responsible_read_but: gtk::ToggleButton::new_with_label("Read"),
            perm_responsible_write_but: gtk::ToggleButton::new_with_label("Write"),
            perm_responsible_comment_but: gtk::ToggleButton::new_with_label("Comment"),
            permission_others_label: gtk::Label::new(Some("Permissions of  others:")),
            perm_others_button_box: gtk::ButtonBox::new(gtk::Orientation::Horizontal),
            perm_others_read_but: gtk::ToggleButton::new_with_label("Read"),
            perm_others_write_but: gtk::ToggleButton::new_with_label("Write"),
            perm_others_comment_but: gtk::ToggleButton::new_with_label("Comment"),

            data_entry: gtk::Entry::new(),
            scrolled_window_label: gtk::Label::new(Some("Commentaries:")),
            scrolled_window: gtk::ScrolledWindow::new(None, None),
            comments_text: gtk::TextView::new(),
            responsible_label: gtk::Label::new(Some("Responsible:")),
            responsible_combobox: gtk::ComboBoxText::new(),

            button_box: gtk::ButtonBox::new(gtk::Orientation::Horizontal),
            ok_button: gtk::Button::new_with_mnemonic("_Ok"),
            cancel_button: gtk::Button::new_with_mnemonic("_Cancel"),
            doc: Rc::new(RefCell::new(document)),
        };

        tmp.setup();
        tmp.connect_signals();
        tmp.pack_and_show();

        tmp
    }

    fn setup(&self) {
        self.dialog_setup();
        self.labels_setup();
        self.name_entry_setup();
        self.author_combobox_setup();
        self.category_combobox_setup();
        self.status_combobox_setup();
        self.date_expired_calendar_setup();

        self.permission_setup();
        self.data_entry_setup();
        self.comments_text_setup();
        self.responsible_combobox_setup();
    }

    fn dialog_setup(&self) {
        use gtk::WindowExt;

        self.dialog.set_title("Edit/View ticket");
        self.dialog.set_transient_for(Some(&self.main_ui.window));
        self.dialog.set_modal(true);

        self.dialog.set_default_size(500, 700);
    }

    fn labels_setup(&self) {
        use gtk::WidgetExt;

        self.name_label.set_halign(gtk::Align::Start);
        self.author_label.set_halign(gtk::Align::Start);
        self.category_label.set_halign(gtk::Align::Start);
        self.status_label.set_halign(gtk::Align::Start);
        self.date_expired_label.set_halign(gtk::Align::Start);
        self.scrolled_window_label.set_halign(gtk::Align::Start);
        self.responsible_label.set_halign(gtk::Align::Start);
    }

    fn name_entry_setup(&self) {
        use gtk::{EntryExt, WidgetExt};

        self.name_entry.set_text(self.doc.borrow().name());
        if self.main_ui.current_user.borrow().is_access_grant(&self.doc.borrow()) {
            self.name_entry.set_sensitive(true);
        } else {
            self.name_entry.set_sensitive(false);
        }
    }

    fn author_combobox_setup(&self) {
        use gtk::{ComboBoxExt, WidgetExt};

        self.author_combobox.append_text(self.doc.borrow().metadata().author().name());
        self.author_combobox.set_active(0);

        match User::get_users(&self.main_ui.db.borrow()) {
            Ok(users) => {
                for user in users {
                    if user.name() != self.doc.borrow().metadata().author().name() {
                        self.author_combobox.append_text(user.name());
                    }
                }
            }
            Err(e) => {
                show_error_dialog(&self.dialog,
                                  &format!("Error of reading users from database.\n{}", e));
            }
        }

        if self.main_ui.current_user.borrow().is_root() {
            self.author_combobox.set_sensitive(true);
        } else {
            self.author_combobox.set_sensitive(false);
        }
    }

    fn category_combobox_setup(&self) {
        use gtk::{ComboBoxExt, WidgetExt};

        self.category_combobox.append_text(self.doc.borrow().metadata().category().name());
        self.category_combobox.set_active(0);

        match Category::get_categories(&self.main_ui.db.borrow()) {
            Ok(categories) => {
                for category in categories {
                    if category.name() != self.doc.borrow().metadata().category().name() {
                        self.category_combobox.append_text(category.name());
                    }
                }
            }
            Err(e) => {
                show_error_dialog(&self.dialog,
                                  &format!("Error of reading categories from database.\n{}", e));
            }
        }

        if self.main_ui.current_user.borrow().is_access_grant(&self.doc.borrow()) {
            self.category_combobox.set_sensitive(true);
        } else {
            self.category_combobox.set_sensitive(false);
        }
    }

    fn status_combobox_setup(&self) {
        use gtk::{ComboBoxExt, WidgetExt};

        self.status_combobox.append_text("Beginning");
        self.status_combobox.append_text("In progress");
        self.status_combobox.append_text("Complete");
        self.status_combobox.set_active(self.doc.borrow().metadata().status().get_num() as i32);

        if self.main_ui.current_user.borrow().is_access_grant(&self.doc.borrow()) {
            self.status_combobox.set_sensitive(true);
        } else {
            self.status_combobox.set_sensitive(false);
        }
    }

    fn date_expired_calendar_setup(&self) {
        use gtk::WidgetExt;

        self.date_expired_calendar
            .select_month(self.doc.borrow().metadata().date_expired().month(),
                          self.doc.borrow().metadata().date_expired().year() as u32);
        self.date_expired_calendar.select_day(self.doc.borrow().metadata().date_expired().day());

        if self.main_ui.current_user.borrow().is_access_grant(&self.doc.borrow()) {
            self.date_expired_calendar.set_sensitive(true);
        } else {
            self.date_expired_calendar.set_sensitive(false);
        }
    }

    fn permission_setup(&self) {
        use gtk::{ToggleButtonExt, WidgetExt};

        let doc_bor = self.doc.borrow();
        let permission = doc_bor.permission();
        if permission.author().read {
            self.perm_author_read_but.set_active(true);
        }
        if permission.author().write {
            self.perm_author_write_but.set_active(true);
        }
        if permission.author().comment {
            self.perm_author_comment_but.set_active(true);
        }
        if permission.responsible().read {
            self.perm_responsible_read_but.set_active(true);
        }
        if permission.responsible().write {
            self.perm_responsible_write_but.set_active(true);
        }
        if permission.responsible().comment {
            self.perm_responsible_comment_but.set_active(true);
        }
        if permission.others().read {
            self.perm_others_read_but.set_active(true);
        }
        if permission.others().write {
            self.perm_others_write_but.set_active(true);
        }
        if permission.others().comment {
            self.perm_others_comment_but.set_active(true);
        }

        if self.main_ui.current_user.borrow().is_access_grant(&self.doc.borrow()) {
            self.perm_author_button_box.set_sensitive(true);
            self.perm_responsible_button_box.set_sensitive(true);
            self.perm_others_button_box.set_sensitive(true);
        } else {
            self.perm_author_button_box.set_sensitive(false);
            self.perm_responsible_button_box.set_sensitive(false);
            self.perm_others_button_box.set_sensitive(false);
        }
    }

    fn data_entry_setup(&self) {}

    fn comments_text_setup(&self) {
        use gtk::WidgetExt;

        match Comment::get_by_doc_id(&self.main_ui.db.borrow(), self.doc.borrow().id()) {
            Ok(comments) => {
                for comment in comments {
                    let buffer_text = self.comments_text.get_buffer().unwrap();
                    buffer_text.set_text(&format!("{} wrote at {}:\n-----\n{}",
                                                  comment.author().name(),
                                                  comment.c_time().to_rfc2822(),
                                                  comment.text()));
                }
            }
            Err(e) => {
                show_error_dialog(&self.dialog,
                                  &format!("Error of reading comments from database.\n{}", e));
            }
        }

        self.comments_text.set_sensitive(false);
    }

    fn responsible_combobox_setup(&self) {
        use gtk::{ComboBoxExt, WidgetExt};

        self.responsible_combobox.append_text(self.doc.borrow().responsible().name());
        self.responsible_combobox.set_active(0);

        match User::get_users(&self.main_ui.db.borrow()) {
            Ok(users) => {
                for user in users {
                    if user.name() != self.doc.borrow().responsible().name() {
                        self.responsible_combobox.append_text(user.name());
                    }
                }
            }
            Err(e) => {
                show_error_dialog(&self.dialog,
                                  &format!("Error of reading users from database.\n{}", e));
            }

        }

        if self.main_ui.current_user.borrow().is_access_grant(&self.doc.borrow()) {
            self.responsible_combobox.set_sensitive(true);
        } else {
            self.responsible_combobox.set_sensitive(false);
        }
    }

    fn connect_signals(&self) {
        self.ok_button_connect();
        self.cancel_button_connect();
    }

    fn ok_button_connect(&self) {
        use gtk::{ButtonExt, EntryExt, ComboBoxExt, ToggleButtonExt, WidgetExt};

        let rc: EditTicket = self.clone();
        self.ok_button.connect_clicked(move |_| {
            let name = rc.name_entry.get_text().unwrap();
            if name.is_empty() {
                show_error_dialog(&rc.dialog, "Name of ticket is empty.");
                return;
            }
            let author = rc.author_combobox.get_active_text().unwrap();
            let category = rc.category_combobox.get_active_text().unwrap();
            let status = Status::from_num(rc.status_combobox.get_active() as i64);
            let date_expired = rc.date_expired_calendar.get_date();
            let date_expired = UTC.ymd(date_expired.0 as i32, date_expired.1, date_expired.2)
                .and_hms(0, 0, 0);
            let author_perm = NaivePermission::new(rc.perm_author_read_but.get_active(),
                                                   rc.perm_author_write_but.get_active(),
                                                   rc.perm_author_comment_but.get_active());
            let responsible_perm = NaivePermission::new(rc.perm_responsible_read_but.get_active(),
                                                        rc.perm_responsible_write_but.get_active(),
                                                        rc.perm_responsible_comment_but
                                                            .get_active());
            let others_perm = NaivePermission::new(rc.perm_others_read_but.get_active(),
                                                   rc.perm_others_write_but.get_active(),
                                                   rc.perm_others_comment_but.get_active());
            let permission = Permission::from_naive(author_perm, responsible_perm, others_perm);
            let responsible = rc.responsible_combobox.get_active_text().unwrap();

            let mut changed = false;

            let name_changed = rc.doc.borrow().name() != name;
            if name_changed {
                rc.doc.borrow_mut().set_name(name);
                changed = true;
            }

            let author_changed = rc.doc.borrow().metadata().author().name() != author;
            if author_changed {
                rc.doc
                    .borrow_mut()
                    .metadata_mut()
                    .set_author(User::get_user(&rc.main_ui.db.borrow(), &author).unwrap());
                changed = true;
            }

            let category_changed = rc.doc.borrow().metadata().category().name() != category;
            if category_changed {
                rc.doc
                    .borrow_mut()
                    .metadata_mut()
                    .set_category(Category::get_category(&rc.main_ui.db.borrow(), &category)
                        .unwrap());
                changed = true;
            }

            let status_changed = rc.doc.borrow().metadata().status() != status;
            if status_changed {
                rc.doc.borrow_mut().metadata_mut().set_status(status);
                changed = true;
            }

            let date_expired_changed = rc.doc.borrow().metadata().date_expired() != date_expired;
            if date_expired_changed {
                rc.doc
                    .borrow_mut()
                    .metadata_mut()
                    .set_date_expired(date_expired);
                changed = true;
            }

            let permission_changed = rc.doc.borrow().permission() != permission;
            if permission_changed {
                rc.doc.borrow_mut().set_permission(permission);
                changed = true;
            }

            let responsible_changed = rc.doc.borrow().responsible().name() != responsible;
            if responsible_changed {
                rc.doc
                    .borrow_mut()
                    .set_responsible(User::get_user(&rc.main_ui.db.borrow(), &responsible)
                        .unwrap());
                changed = true;
            }

            if changed {
                let doc_bor_mut = rc.doc.borrow_mut();
                match doc_bor_mut.update(&rc.main_ui.db.borrow()) {
                    Ok(_) => {
                        match doc_bor_mut.metadata().update(&rc.main_ui.db.borrow()) {
                            Ok(_) => {
                                rc.dialog.destroy();
                                rc.main_ui.update_ui();
                            }
                            Err(e) => {
                                show_error_dialog(&rc.dialog,
                                                  &format!("Error of updating document.\n{}", e));
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        show_error_dialog(&rc.dialog,
                                          &format!("Error of updating document.\n{}", e));
                        return;
                    }
                }
            } else {
                rc.dialog.destroy();
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
        self.perm_author_button_box_pack();
        self.perm_responsible_button_box_pack();
        self.perm_others_button_box_pack();
        self.button_box_pack();
        self.scrolled_window_pack();
        self.dialog_pack();
    }

    fn perm_author_button_box_pack(&self) {
        use gtk::ContainerExt;

        self.perm_author_button_box.add(&self.perm_author_read_but);
        self.perm_author_button_box.add(&self.perm_author_write_but);
        self.perm_author_button_box.add(&self.perm_author_comment_but);
    }

    fn perm_responsible_button_box_pack(&self) {
        use gtk::ContainerExt;

        self.perm_responsible_button_box.add(&self.perm_responsible_read_but);
        self.perm_responsible_button_box.add(&self.perm_responsible_write_but);
        self.perm_responsible_button_box.add(&self.perm_responsible_comment_but);
    }

    fn perm_others_button_box_pack(&self) {
        use gtk::ContainerExt;

        self.perm_others_button_box.add(&self.perm_others_read_but);
        self.perm_others_button_box.add(&self.perm_others_write_but);
        self.perm_others_button_box.add(&self.perm_others_comment_but);
    }

    fn button_box_pack(&self) {
        use gtk::ContainerExt;

        self.button_box.add(&self.ok_button);
        self.button_box.add(&self.cancel_button);
    }

    fn scrolled_window_pack(&self) {
        use gtk::ContainerExt;

        self.scrolled_window.add(&self.comments_text);
    }

    fn dialog_pack(&self) {
        use gtk::{DialogExt, BoxExt, WidgetExt};

        let area = self.dialog.get_content_area();


        area.pack_start(&self.name_label, false, false, 0);
        area.pack_start(&self.name_entry, false, false, 0);
        // area.pack_start(&self.c_time_label, false, false, 0);
        // area.pack_start(&self.m_time_label, false, false, 0);
        area.pack_start(&self.author_label, false, false, 0);
        area.pack_start(&self.author_combobox, false, false, 0);
        area.pack_start(&self.category_label, false, false, 0);
        area.pack_start(&self.category_combobox, false, false, 0);
        area.pack_start(&self.status_label, false, false, 0);
        area.pack_start(&self.status_combobox, false, false, 0);
        area.pack_start(&self.date_expired_label, false, false, 0);
        area.pack_start(&self.date_expired_calendar, false, false, 0);
        area.pack_start(&self.permission_author_label, false, false, 0);
        area.pack_start(&self.perm_author_button_box, false, false, 0);
        area.pack_start(&self.permission_responsible_label, false, false, 0);
        area.pack_start(&self.perm_responsible_button_box, false, false, 0);
        area.pack_start(&self.permission_others_label, false, false, 0);
        area.pack_start(&self.perm_others_button_box, false, false, 0);
        // area.pack_start(&self.data_entry, false, false, 0);
        area.pack_start(&self.scrolled_window_label, false, false, 0);
        area.pack_start(&self.scrolled_window, true, true, 0);
        area.pack_start(&self.responsible_label, false, false, 0);
        area.pack_start(&self.responsible_combobox, false, false, 0);
        area.pack_start(&self.button_box, false, false, 0);

        self.dialog.show_all();
    }
}
