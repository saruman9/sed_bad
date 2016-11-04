//! GUI on GTK3
//!
//! TODO Write documentation.

mod auth;

use gtk;
pub use super::user::User;

use std::rc::Rc;
use std::cell::RefCell;

pub struct MainUI {
    // Menu.
    current_user: Rc<RefCell<User>>,

    menu_bar: gtk::MenuBar,
    file_menu_item: gtk::MenuItem,
    file_menu: gtk::Menu,
    quit_menu_item: gtk::MenuItem,

    window: gtk::Window,
    v_box: gtk::Box,
}

impl MainUI {
    pub fn init() -> Rc<RefCell<Self>> {
        gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

        let tmp = MainUI {
            current_user: Rc::new(RefCell::new(User::default())),

            menu_bar: gtk::MenuBar::new(),
            file_menu_item: gtk::MenuItem::new_with_mnemonic("_File"),
            file_menu: gtk::Menu::new(),
            quit_menu_item: gtk::MenuItem::new_with_mnemonic("_Quit"),

            window: gtk::Window::new(gtk::WindowType::Toplevel),
            v_box: gtk::Box::new(gtk::Orientation::Vertical, 0),
        };
        tmp.setup();
        tmp.connect_signals();
        tmp.pack_and_show();
        Rc::new(RefCell::new(tmp))
    }

    pub fn run(&self) {
        gtk::main();
    }

    pub fn set_user(&self, user: User) {
        self.current_user.borrow_mut().set(user);
    }

    fn setup(&self) {
        self.setup_menu();
        self.setup_v_box();
        self.setup_window();
    }

    fn setup_menu(&self) {

    }

    fn setup_v_box(&self) {

    }

    fn setup_window(&self) {
        use gtk::WindowExt;

        self.window.set_title("СЭД-БЭД");
        self.window.set_default_size(1000, 590);
    }

    fn connect_signals(&self) {
        self.connect_signals_window();
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
        self.pack_menu_bar();
    }

    fn pack_file_menu(&self) {
        use gtk::{MenuItemExt, MenuShellExt};

        self.file_menu_item.set_submenu(Some(&self.file_menu));
        self.file_menu.append(&self.quit_menu_item);
    }

    fn pack_menu_bar(&self) {
        use gtk::MenuShellExt;

        self.menu_bar.append(&self.file_menu_item);
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
}
