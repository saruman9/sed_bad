//! Dialog for editing information about user.
//!
//! TODO Write docs.

use gtk;

use super::user_administration::UserAdministration;
use user::User;
use super::utils::show_error_dialog;

#[derive(Clone)]
pub struct EditUser {
    user_adm: UserAdministration,
    dialog: gtk::Dialog,

    id_entry: gtk::Entry,
    name_entry: gtk::Entry,
    pass_entry: gtk::Entry,
}

impl EditUser {
    pub fn new(user_adm: UserAdministration) -> Self {
        let tmp = EditUser {
            user_adm: user_adm,
            dialog: gtk::Dialog::new(),

            id_entry: gtk::Entry::new(),
            name_entry: gtk::Entry::new(),
            pass_entry: gtk::Entry::new(),
        };
        tmp.setup();
        tmp.connect_signals();
        tmp.pack_and_show();

        tmp
    }

    fn setup(&self) {
        self.setup_entries();
        self.setup_dialog();
    }

    fn setup_entries(&self) {
        use gtk::{TreeModelExt, WidgetExt, EntryExt};

        if let Some((tree_model, tree_iter)) = self.user_adm
            .list_users_tree_view
            .get_selection()
            .get_selected() {

            // TODO Critical unwrap!
            self.id_entry.set_text(tree_model.get_value(&tree_iter, 0)
                .get::<String>()
                .unwrap()
                .as_ref());

            self.id_entry.set_sensitive(false);

            self.name_entry.set_text(tree_model.get_value(&tree_iter, 1)
                .get::<String>()
                .unwrap()
                .as_ref());

            self.pass_entry.set_text(tree_model.get_value(&tree_iter, 2)
                .get::<String>()
                .unwrap()
                .as_ref());
        }
    }

    fn setup_dialog(&self) {
        use gtk::WindowExt;

        self.dialog.set_title("Edit user");
        self.dialog.set_transient_for(Some(&self.user_adm.dialog));
        self.dialog.set_modal(true);
    }

    fn connect_signals(&self) {
        self.connect_signals_entries();
    }

    fn connect_signals_entries(&self) {
        use gtk::{EntryExt, WidgetExt};

        {
            let rc = self.clone();
            self.name_entry.connect_activate(move |_| {
                let id = rc.id_entry
                    .get_text()
                    .map(|id| id.parse().unwrap())
                    .unwrap();
                if id == 1 {
                    show_error_dialog(&rc.dialog, "Root unchanged!");
                    return;
                }
                let name = rc.name_entry.get_text().unwrap();
                let pass = rc.pass_entry.get_text().unwrap();
                if let Err(e) = User::update_by_id(&rc.user_adm.main_ui.db.borrow(),
                                                   id,
                                                   &name,
                                                   &pass) {
                    show_error_dialog(&rc.dialog,
                                      &format!("Error of editing user's information.\n{}", e));
                } else {
                    rc.dialog.destroy();
                    rc.user_adm.update_ui();
                }
            });
        }

        {
            let rc = self.clone();
            self.pass_entry.connect_activate(move |_| {
                let id = rc.id_entry
                    .get_text()
                    .map(|id| id.parse().unwrap())
                    .unwrap();
                let name = rc.name_entry.get_text().unwrap();
                let pass = rc.pass_entry.get_text().unwrap();
                if let Err(e) = User::update_by_id(&rc.user_adm.main_ui.db.borrow(),
                                                   id,
                                                   &name,
                                                   &pass) {
                    show_error_dialog(&rc.dialog,
                                      &format!("Error of editing user's information.\n{}", e));
                } else {
                    rc.dialog.destroy();
                    rc.user_adm.update_ui();
                }
            });
        }
    }

    fn pack_and_show(&self) {
        self.pack_dialog();
    }

    fn pack_dialog(&self) {
        use gtk::{DialogExt, BoxExt, WidgetExt};

        let area = self.dialog.get_content_area();
        area.pack_start(&self.id_entry, false, false, 0);
        area.pack_start(&self.name_entry, false, false, 0);
        area.pack_start(&self.pass_entry, false, false, 0);
        self.dialog.show_all();
    }
}
