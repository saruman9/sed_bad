#![warn(missing_docs)]
//! Authorization dialog on start of program.

use gtk;

use super::MainUI;
use user::User;

#[derive(Clone)]
pub struct AuthUI {
    main_ui: MainUI,
    dialog: gtk::Dialog,

    info_bar: gtk::InfoBar,
    info_label: gtk::Label,

    grid: gtk::Grid,
    user_label: gtk::Label,
    user_entry: gtk::Entry,
    pass_label: gtk::Label,
    pass_entry: gtk::Entry,

    button_box: gtk::ButtonBox,
    login_button: gtk::Button,
    exit_button: gtk::Button,
}

impl AuthUI {
    pub fn init(main_ui: MainUI) -> Self {
        let tmp = AuthUI {
            main_ui: main_ui,
            dialog: gtk::Dialog::new(),

            info_bar: gtk::InfoBar::new(),
            info_label: gtk::Label::new(Some("")),

            grid: gtk::Grid::new(),
            user_label: gtk::Label::new(Some("User:")),
            user_entry: gtk::Entry::new(),
            pass_label: gtk::Label::new(Some("Password:")),
            pass_entry: gtk::Entry::new(),

            button_box: gtk::ButtonBox::new(gtk::Orientation::Horizontal),
            login_button: gtk::Button::new_with_label("Log in"),
            exit_button: gtk::Button::new_with_label("Exit"),
        };
        tmp.setup();
        tmp.connect_signals();
        tmp.pack_and_show();
        tmp
    }

    fn setup(&self) {
        self.setup_info_bar();
        self.setup_labels();
        self.setup_entries();
        self.setup_grid();
        self.setup_button_box();
        self.setup_dialog();
    }

    fn setup_info_bar(&self) {
        use gtk::WidgetExt;

        self.info_bar.set_no_show_all(true);
    }

    fn setup_labels(&self) {
        use gtk::WidgetExt;

        self.user_label.set_halign(gtk::Align::Start);
        self.pass_label.set_halign(gtk::Align::Start);
    }

    fn setup_entries(&self) {
        use gtk::EntryExt;

        self.pass_entry.set_visibility(false);
        self.pass_entry.set_input_purpose(gtk::InputPurpose::Password);
    }

    fn setup_grid(&self) {
        self.grid.set_row_spacing(5);
        self.grid.set_column_spacing(5);
    }

    fn setup_button_box(&self) {
        self.button_box.set_layout(gtk::ButtonBoxStyle::End);
    }

    fn setup_dialog(&self) {
        use gtk::WindowExt;

        // self.dialog.set_default_size()
        self.dialog.set_title("Authorization");
        self.dialog.set_transient_for(Some(&self.main_ui.window));
        self.dialog.set_modal(true);
        self.dialog.set_position(gtk::WindowPosition::Center);
    }

    fn connect_signals(&self) {
        self.connect_signals_entries();
        self.connect_signals_dialog();
        self.connect_signals_exit_button();
        self.connect_signals_log_in_button();
    }

    fn connect_signals_entries(&self) {
        use gtk::{EntryExt, ButtonExt};

        {
            let rc: AuthUI = self.clone();
            self.user_entry.connect_activate(move |_| {
                rc.login_button.clicked();
            });
        }
        {
            let rc: AuthUI = self.clone();
            self.pass_entry.connect_activate(move |_| {
                rc.login_button.clicked();
            });
        }
    }

    fn connect_signals_dialog(&self) {
        use gtk::WidgetExt;

        self.dialog.connect_delete_event(move |d, _| {
            d.destroy();
            gtk::Inhibit(false)
        });
    }

    fn connect_signals_exit_button(&self) {
        use gtk::{ButtonExt, WidgetExt};

        let rc: AuthUI = self.clone();
        self.exit_button.connect_clicked(move |_| {
            rc.dialog.destroy();
            rc.main_ui.update_ui();
        });
    }

    fn connect_signals_log_in_button(&self) {
        use gtk::{ButtonExt, EntryExt, WidgetExt};

        let rc: AuthUI = self.clone();
        self.login_button.connect_clicked(move |_| {
            if let Some(name) = rc.user_entry.get_text() {
                if let Some(pass) = rc.pass_entry.get_text() {
                    if name.is_empty() {
                        rc.info_label.set_label("Enter username.");
                        rc.info_bar.show();
                        return;
                    }
                    if pass.is_empty() {
                        rc.info_label.set_label("Enter the password.");
                        rc.info_bar.show();
                        return;
                    }
                    let new_user = User::new(name, pass);
                    match new_user.exists(&rc.main_ui.db.borrow()) {
                        Ok(b) => {
                            if b {
                                rc.main_ui.current_user.borrow_mut().set(new_user);
                            } else {
                                rc.info_label.set_label("Error of authorization.");
                                rc.info_bar.show();
                                return;
                            }
                        }
                        Err(e) => {
                            rc.info_label.set_label(format!("Error of database.\n{}", e).as_ref());
                            rc.info_bar.show();
                            return;
                        }

                    }
                    rc.main_ui.update_ui();
                    rc.dialog.destroy();
                }
            }
        });
    }

    fn pack_and_show(&self) {
        self.pack_info_bar();
        self.pack_user();
        self.pack_pass();
        self.pack_button_box();
        self.pack_dialog();
    }

    fn pack_info_bar(&self) {
        use gtk::{ContainerExt, WidgetExt};

        self.info_bar.add(&self.info_label);
        self.info_label.show();
    }

    fn pack_user(&self) {
        self.grid.attach(&self.user_label, 0, 0, 1, 1);
        self.grid.attach(&self.user_entry, 1, 0, 1, 1);
    }

    fn pack_pass(&self) {
        self.grid.attach(&self.pass_label, 0, 1, 1, 1);
        self.grid.attach(&self.pass_entry, 1, 1, 1, 1);
    }

    fn pack_button_box(&self) {
        use gtk::BoxExt;

        self.button_box.pack_start(&self.login_button, false, false, 0);
        self.button_box.pack_start(&self.exit_button, false, false, 0);
    }

    fn pack_dialog(&self) {
        use gtk::{WidgetExt, DialogExt, BoxExt};

        let area = self.dialog.get_content_area();
        area.pack_start(&self.info_bar, false, false, 0);
        area.pack_start(&self.grid, true, true, 5);
        area.pack_start(&self.button_box, false, false, 0);

        self.dialog.show_all();
    }
}
