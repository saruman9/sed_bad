//! Dialog of user administration by root.
//!
//! TODO Write documentation.

use gtk;

use super::MainUI;
use user::User;
use super::utils::show_error_dialog;
use super::edit_user::EditUser;

#[derive(Clone)]
pub struct UserAdministration {
    pub main_ui: MainUI,

    pub dialog: gtk::Dialog,
    pub main_v_box: gtk::Box,

    pub new_user_entry: gtk::Entry,

    pub scrolled_window: gtk::ScrolledWindow,

    pub list_users_tree_view: gtk::TreeView,
    pub list_users_store: gtk::ListStore,
    pub id_column: gtk::TreeViewColumn,
    pub name_column: gtk::TreeViewColumn,
    pub pass_column: gtk::TreeViewColumn,
    pub pass_hash_column: gtk::TreeViewColumn,
    pub id_cell: gtk::CellRendererText,
    pub name_cell: gtk::CellRendererText,
    pub pass_cell: gtk::CellRendererText,
    pub pass_hash_cell: gtk::CellRendererText,

    pub button_box: gtk::ButtonBox,
    pub edit_button: gtk::Button,
    pub delete_button: gtk::Button,
}

impl UserAdministration {
    pub fn new(main_ui: MainUI) -> Self {
        let tmp = UserAdministration {
            main_ui: main_ui.clone(),

            dialog: gtk::Dialog::new_with_buttons(Some("User administration"),
                                                  Some(&main_ui.window),
                                                  gtk::DIALOG_MODAL,
                                                  &[("Close", gtk::ResponseType::Close.into())]),
            main_v_box: gtk::Box::new(gtk::Orientation::Vertical, 0),

            new_user_entry: gtk::Entry::new(),

            scrolled_window: gtk::ScrolledWindow::new(None, None),

            list_users_tree_view: gtk::TreeView::new(),
            list_users_store: gtk::ListStore::new(&[gtk::Type::String, // Id.
                                                    gtk::Type::String, // Name.
                                                    gtk::Type::String, // Pass.
                                                    gtk::Type::String]), // Pass Hash.
            id_column: gtk::TreeViewColumn::new(),
            name_column: gtk::TreeViewColumn::new(),
            pass_column: gtk::TreeViewColumn::new(),
            pass_hash_column: gtk::TreeViewColumn::new(),
            id_cell: gtk::CellRendererText::new(),
            name_cell: gtk::CellRendererText::new(),
            pass_cell: gtk::CellRendererText::new(),
            pass_hash_cell: gtk::CellRendererText::new(),

            button_box: gtk::ButtonBox::new(gtk::Orientation::Horizontal),
            edit_button: gtk::Button::new_with_label("Edit"),
            delete_button: gtk::Button::new_with_label("Delete"),
        };
        tmp.setup();
        tmp.connect_signals();
        tmp.pack_and_show();
        tmp
    }

    fn setup(&self) {
        self.setup_new_user_entry();
        self.setup_columns();
        self.setup_list_users_store();
        self.setup_button_box();
        self.setup_dialog();
    }

    fn setup_new_user_entry(&self) {
        use gtk::EntryExt;

        self.new_user_entry.set_placeholder_text(Some("New user..."));
    }

    fn setup_columns(&self) {
        // Set title.
        self.id_column.set_title("Id");
        self.name_column.set_title("Name");
        self.pass_column.set_title("Pass");
        self.pass_hash_column.set_title("Pass hash");

        // Set resizable.
        self.id_column.set_resizable(true);
        self.name_column.set_resizable(true);
        self.pass_column.set_resizable(true);
        self.pass_hash_column.set_resizable(true);

        // Attach cell to column.
        self.id_column.pack_start(&self.id_cell, true);
        self.name_column.pack_start(&self.name_cell, true);
        self.pass_column.pack_start(&self.pass_cell, true);
        self.pass_hash_column.pack_start(&self.pass_hash_cell, true);

        // Add attribute.
        self.id_column.add_attribute(&self.id_cell, "text", 0);
        self.name_column.add_attribute(&self.name_cell, "text", 1);
        self.pass_column.add_attribute(&self.pass_cell, "text", 2);
        self.pass_hash_column.add_attribute(&self.pass_hash_cell, "text", 3);

        // Set clickable.
        self.id_column.set_clickable(true);
        self.name_column.set_clickable(true);
        self.pass_column.set_clickable(true);
        self.pass_hash_column.set_clickable(true);

        // Set reorderable.
        self.id_column.set_reorderable(true);
        self.name_column.set_reorderable(true);
        self.pass_column.set_reorderable(true);
        self.pass_hash_column.set_reorderable(true);

        // Set sort column.
        self.id_column.set_sort_column_id(0);
        self.name_column.set_sort_column_id(1);
        self.pass_column.set_sort_column_id(2);
        self.pass_hash_column.set_sort_column_id(3);

        self.list_users_tree_view.append_column(&self.id_column);
        self.list_users_tree_view.append_column(&self.name_column);
        self.list_users_tree_view.append_column(&self.pass_column);
        self.list_users_tree_view.append_column(&self.pass_hash_column);
    }

    fn setup_list_users_store(&self) {
        match User::get_users(&self.main_ui.db.borrow()) {
            Ok(users) => {
                for user in users {
                    self.list_users_store.insert_with_values(None,
                                                             &[0, 1, 2, 3],
                                                             &[&user.id(),
                                                               &user.name(),
                                                               &user.pass(),
                                                               &user.pass_hash()]);
                }
                self.list_users_tree_view.set_model(Some(&self.list_users_store));
            }
            Err(e) => {
                show_error_dialog(&self.dialog,
                                  &format!("Error of reading from database (`users` table).\n{}",
                                           e));
            }
        }
    }

    fn setup_button_box(&self) {
        self.button_box.set_layout(gtk::ButtonBoxStyle::Spread);
    }

    fn setup_dialog(&self) {
        use gtk::WindowExt;

        self.dialog.set_default_size(500, 600);
        self.dialog.set_focus(Some(&self.list_users_tree_view));
    }

    fn connect_signals(&self) {
        self.connect_signals_dialog();
        self.connect_signals_new_user_entry();
        self.connect_signals_delete_button();
        self.connect_signals_edit_button();
        self.connect_signals_row_activated();
    }

    fn connect_signals_dialog(&self) {
        use gtk::{DialogExt, WidgetExt};

        self.dialog.connect_close(|dialog| {
            dialog.destroy();
        });

        self.dialog.connect_delete_event(|dialog, _| {
            dialog.destroy();
            gtk::Inhibit(false)
        });
    }

    fn connect_signals_new_user_entry(&self) {
        use gtk::EntryExt;

        let rc = self.clone();
        self.new_user_entry.connect_activate(move |_| {
            if let Some(new_username) = rc.new_user_entry.get_text() {
                let mut new_user = User::new(new_username.clone(), new_username);
                match new_user.save_to_db(&rc.main_ui.db.borrow()) {
                    Ok(_) => {
                        rc.list_users_store.insert_with_values(None,
                                                               &[0, 1, 2, 3],
                                                               &[&new_user.id(),
                                                                 &new_user.name(),
                                                                 &new_user.pass(),
                                                                 &new_user.pass_hash()]);
                        rc.update_ui();
                    }
                    Err(e) => {
                        show_error_dialog(&rc.dialog,
                                          &format!("Error of creating new user.\n{}", e));
                    }
                }
            }
        });
    }

    fn connect_signals_delete_button(&self) {
        use gtk::ButtonExt;

        let rc = self.clone();
        self.delete_button.connect_clicked(move |_| {
            if let Some((tree_model, tree_iter)) = rc.list_users_tree_view
                .get_selection()
                .get_selected() {
                use gtk::TreeModelExt;

                // TODO Critical unwrap!
                let user_id: i64 = tree_model.get_value(&tree_iter, 0)
                    .get::<String>()
                    .map(|id| id.parse().unwrap())
                    .unwrap();
                if user_id != 1 {
                    match User::delete_by_id(&rc.main_ui.db.borrow(), user_id) {
                        Ok(_) => {
                            rc.list_users_store.remove(&tree_iter);
                            rc.update_ui();
                            rc.main_ui.update_ui();
                        }
                        Err(e) => {
                            show_error_dialog(&rc.dialog,
                                              &format!("Error of deleting user.\n{}", e));
                        }
                    }
                } else {
                    show_error_dialog(&rc.dialog, "Root unchanged!");
                    return;
                }
            } else {
                show_error_dialog(&rc.dialog, "No one row is selecting.");
            }
        });
    }

    fn connect_signals_edit_button(&self) {
        use gtk::ButtonExt;

        let rc: UserAdministration = self.clone();
        self.edit_button.connect_clicked(move |_| {
            if rc.list_users_tree_view
                .get_selection()
                .get_selected()
                .is_some() {
                EditUser::new(rc.clone());
            } else {
                show_error_dialog(&rc.dialog, "No one row is selecting.");
            }
        });
    }

    fn connect_signals_row_activated(&self) {
        use gtk::TreeViewSignals;

        let rc = self.clone();
        self.list_users_tree_view.connect_row_activated(move |_, _, _| {
            EditUser::new(rc.clone());
        });
    }

    fn pack_and_show(&self) {
        self.pack_scrolled_window();
        self.pack_button_box();
        self.pack_main_v_box();
        self.pack_dialog();
    }

    fn pack_scrolled_window(&self) {
        use gtk::ContainerExt;

        self.scrolled_window.add(&self.list_users_tree_view);
    }

    fn pack_main_v_box(&self) {
        use gtk::BoxExt;

        self.main_v_box.pack_start(&self.new_user_entry, false, false, 10);
        self.main_v_box.pack_start(&self.scrolled_window, true, true, 10);
        self.main_v_box.pack_start(&self.button_box, false, false, 10);
    }

    fn pack_button_box(&self) {
        use gtk::BoxExt;

        self.button_box.pack_start(&self.edit_button, false, false, 0);
        self.button_box.pack_start(&self.delete_button, false, false, 0);
    }

    fn pack_dialog(&self) {
        use gtk::{DialogExt, BoxExt, WidgetExt};

        let area = self.dialog.get_content_area();
        area.pack_start(&self.main_v_box, true, true, 0);

        self.dialog.show_all();
        let response = self.dialog.run();
        if response == gtk::ResponseType::Close.into() {
            self.dialog.destroy();
        }
    }

    pub fn update_ui(&self) {
        use gtk::EntryExt;

        self.new_user_entry.set_text("");
        self.list_users_store.clear();
        self.setup_list_users_store();
    }
}
