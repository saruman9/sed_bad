//! GUI on GTK3
//!
//! TODO Write documentation.

mod auth;
mod user_administration;
mod edit_user;
mod utils;

use gtk;

use std::rc::Rc;
use std::cell::RefCell;

use user::User;
use db::Db;

#[derive(Clone)]
pub struct MainUI {
    // Menu.
    current_user: Rc<RefCell<User>>,
    db: Rc<RefCell<Db>>,

    menu_bar: gtk::MenuBar,

    file_menu_item: gtk::MenuItem,
    file_menu: gtk::Menu,
    log_in_menu_item: gtk::MenuItem,
    print_user_menu_item: gtk::MenuItem,
    quit_menu_item: gtk::MenuItem,

    root_menu_item: gtk::MenuItem,
    root_menu: gtk::Menu,
    user_administration_menu_item: gtk::MenuItem,
    category_administration_menu_item: gtk::MenuItem,

    window: gtk::Window,
    v_box: gtk::Box,
}

impl MainUI {
    pub fn init() -> Self {
        gtk::init().expect("Failed to initialize GTK.");

        let tmp = MainUI {
            current_user: Rc::new(RefCell::new(User::default())),
            db: Rc::new(RefCell::new(Db::new()
                .and_then(|d| d.init_root())
                .expect("Error of creating database."))),

            menu_bar: gtk::MenuBar::new(),

            file_menu_item: gtk::MenuItem::new_with_mnemonic("_File"),
            file_menu: gtk::Menu::new(),
            log_in_menu_item: gtk::MenuItem::new_with_mnemonic("_Log in"),
            print_user_menu_item: gtk::MenuItem::new_with_mnemonic("_Print User"),
            quit_menu_item: gtk::MenuItem::new_with_mnemonic("_Quit"),

            root_menu_item: gtk::MenuItem::new_with_mnemonic("_Root configuration"),
            root_menu: gtk::Menu::new(),
            user_administration_menu_item: gtk::MenuItem::new_with_mnemonic("_User administration"),
            category_administration_menu_item: gtk::MenuItem::new_with_mnemonic("_Category \
                                                                                 administration"),

            window: gtk::Window::new(gtk::WindowType::Toplevel),
            v_box: gtk::Box::new(gtk::Orientation::Vertical, 0),
        };
        tmp.setup();
        tmp.connect_signals();
        tmp.pack_and_show();

        tmp
    }

    pub fn run(&self) {
        gtk::main();
    }

    fn setup(&self) {
        self.setup_menu();
        self.setup_v_box();
        self.setup_window();
    }

    fn setup_menu(&self) {
        self.setup_root_menu();
    }

    fn setup_root_menu(&self) {
        use gtk::WidgetExt;

        self.root_menu_item.hide();
        self.root_menu_item.set_no_show_all(true);
    }

    fn setup_v_box(&self) {}

    fn setup_window(&self) {
        use gtk::WindowExt;

        self.window.set_title("СЭД-БЭД");
        self.window.set_default_size(1000, 590);
    }

    fn connect_signals(&self) {
        self.connect_signals_quit_menu();
        self.connect_signals_log_in_menu();
        self.connect_signals_print_user_menu();
        self.connect_signals_user_administration_menu();
        self.connect_signals_window();
    }

    fn connect_signals_log_in_menu(&self) {
        use gtk::MenuItemExt;

        let rc = self.clone();
        self.log_in_menu_item.connect_activate(move |_| {
            auth::AuthUI::init(rc.clone());
        });
    }

    fn connect_signals_print_user_menu(&self) {
        use gtk::MenuItemExt;

        let rc = self.clone();
        self.print_user_menu_item.connect_activate(move |_| {
            println!("{:?}", rc.current_user);
        });
    }

    fn connect_signals_quit_menu(&self) {
        use gtk::MenuItemExt;

        self.quit_menu_item.connect_activate(|_| {
            gtk::main_quit();
        });
    }

    fn connect_signals_user_administration_menu(&self) {
        use gtk::MenuItemExt;

        let rc = self.clone();
        self.user_administration_menu_item.connect_activate(move |_| {
            user_administration::UserAdministration::new(rc.clone());
        });
    }

    fn connect_signals_window(&self) {
        use gtk::WidgetExt;

        self.window.connect_delete_event(|_, _| {
            gtk::main_quit();
            gtk::Inhibit(false)
        });
    }

    fn pack_and_show(&self) {
        self.pack_menu();
        self.pack_v_box();
        self.pack_window();
    }

    fn pack_menu(&self) {
        self.pack_file_menu();
        self.pack_root_menu();
        self.pack_menu_bar();
    }

    fn pack_file_menu(&self) {
        use gtk::{MenuItemExt, MenuShellExt};

        self.file_menu_item.set_submenu(Some(&self.file_menu));
        self.file_menu.append(&self.print_user_menu_item);
        self.file_menu.append(&self.log_in_menu_item);
        self.file_menu.append(&self.quit_menu_item);
    }

    fn pack_root_menu(&self) {
        use gtk::{MenuItemExt, MenuShellExt};

        self.root_menu_item.set_submenu(Some(&self.root_menu));
        self.root_menu.append(&self.user_administration_menu_item);
        self.root_menu.append(&self.category_administration_menu_item);
    }

    fn pack_menu_bar(&self) {
        use gtk::MenuShellExt;

        self.menu_bar.append(&self.file_menu_item);
        self.menu_bar.append(&self.root_menu_item);
    }

    fn pack_v_box(&self) {
        use gtk::BoxExt;

        self.v_box.pack_start(&self.menu_bar, false, false, 0);
    }

    fn pack_window(&self) {
        use gtk::{WidgetExt, ContainerExt};

        self.window.add(&self.v_box);

        self.window.show_all();
    }

    pub fn update_ui(&self) {
        self.update_menu_bar();
        self.update_main();
    }

    fn update_menu_bar(&self) {
        use gtk::WidgetExt;

        if self.current_user.borrow().is_root() {
            self.root_menu.show_all();
            self.root_menu_item.show();
        } else {
            self.root_menu_item.hide();
        }
    }

    fn update_main(&self) {}
}
