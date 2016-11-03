//! GUI on GTK3
//!
//! TODO Write documentation.

use gtk;

use std::rc::Rc;
use std::cell::RefCell;

pub struct MainUI {
    // Menu.
    window: gtk::Window,
    v_box: gtk::Box,
}

impl MainUI {
    pub fn init() -> Rc<RefCell<Self>> {
        gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

        let tmp = MainUI {
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

    fn setup(&self) {
        self.setup_window();
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
        self.pack_window();
    }

    fn pack_window(&self) {
        use gtk::WidgetExt;

        self.window.show_all();
    }
}
