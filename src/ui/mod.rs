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

    toolbar: gtk::Toolbar,
    create_ticket_t_button: gtk::ToolButton,
    create_directory_t_button: gtk::ToolButton,
    open_ticket_t_button: gtk::ToolButton,
    edit_access_t_button: gtk::ToolButton,

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
    h_box: gtk::Box,

    dir_cal_box: gtk::Box,
    directories_list: gtk::ListBox,
    calendar: gtk::Calendar,

    tickets_scrolled_window: gtk::ScrolledWindow,
    tickets_tree_view: gtk::TreeView,
    tickets_list_store: gtk::ListStore,

    id_column: gtk::TreeViewColumn,
    completed_column: gtk::TreeViewColumn,
    name_column: gtk::TreeViewColumn,
    author_column: gtk::TreeViewColumn,
    ctime_column: gtk::TreeViewColumn,
    mtime_column: gtk::TreeViewColumn,
    responsible_column: gtk::TreeViewColumn,

    id_cell: gtk::CellRendererText,
    completed_cell: gtk::CellRendererToggle,
    name_cell: gtk::CellRendererText,
    author_cell: gtk::CellRendererText,
    ctime_cell: gtk::CellRendererText,
    mtime_cell: gtk::CellRendererText,
    responsible_cell: gtk::CellRendererText,
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

            toolbar: gtk::Toolbar::new(),
            create_ticket_t_button: gtk::ToolButton::new(
                Some(
                    &gtk::Image::new_from_icon_name("document-new", gtk::IconSize::LargeToolbar
                                                    .into())
                ),
                Some("Create ticket")),
            create_directory_t_button: gtk::ToolButton::new(
                Some(
                    &gtk::Image::new_from_icon_name("folder-new", gtk::IconSize::LargeToolbar
                                                    .into())
                ),
                Some("Create directory")),
            open_ticket_t_button: gtk::ToolButton::new(
                Some(
                    &gtk::Image::new_from_icon_name("document-open", gtk::IconSize::LargeToolbar
                                                    .into())
                ),
                Some("Open ticket")),
            edit_access_t_button: gtk::ToolButton::new(
                Some(
                    &gtk::Image::new_from_icon_name("dialog-password", gtk::IconSize::LargeToolbar
                                                    .into())
                ),
                Some("Edit access")),

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
            h_box: gtk::Box::new(gtk::Orientation::Horizontal, 0),

            dir_cal_box: gtk::Box::new(gtk::Orientation::Vertical, 0),
            directories_list: gtk::ListBox::new(),
            calendar: gtk::Calendar::new(),

            tickets_scrolled_window: gtk::ScrolledWindow::new(None, None),
            tickets_tree_view: gtk::TreeView::new(),
            tickets_list_store: gtk::ListStore::new(&[gtk::Type::String, // Id.
                                                      gtk::Type::Bool, // Completed.
                                                      gtk::Type::String, // Name.
                                                      gtk::Type::String, // Author.
                                                      gtk::Type::String, // Create time.
                                                      gtk::Type::String, // Modification time.
                                                      gtk::Type::String]), // Responsible.

            id_column: gtk::TreeViewColumn::new(),
            completed_column: gtk::TreeViewColumn::new(),
            name_column: gtk::TreeViewColumn::new(),
            author_column: gtk::TreeViewColumn::new(),
            ctime_column: gtk::TreeViewColumn::new(),
            mtime_column: gtk::TreeViewColumn::new(),
            responsible_column: gtk::TreeViewColumn::new(),

            id_cell: gtk::CellRendererText::new(),
            completed_cell: gtk::CellRendererToggle::new(),
            name_cell: gtk::CellRendererText::new(),
            author_cell: gtk::CellRendererText::new(),
            ctime_cell: gtk::CellRendererText::new(),
            mtime_cell: gtk::CellRendererText::new(),
            responsible_cell: gtk::CellRendererText::new(),
        };
        tmp.setup();
        tmp.connect_signals();
        tmp.pack_and_show();
        tmp.update_ui();

        tmp
    }

    pub fn run(&self) {
        gtk::main();
    }

    fn setup(&self) {
        self.setup_columns();
        self.setup_tickets_list_store();
        self.setup_directories_list();
        self.setup_window();
    }

    fn setup_columns(&self) {
        // Set title.
        self.id_column.set_title("Id");
        self.completed_column.set_title("C");
        self.name_column.set_title("Name");
        self.author_column.set_title("Author");
        self.ctime_column.set_title("Create");
        self.mtime_column.set_title("Modification");
        self.responsible_column.set_title("Responsible");

        // Set resizable.
        self.id_column.set_resizable(true);
        self.completed_column.set_resizable(true);
        self.name_column.set_resizable(true);
        self.author_column.set_resizable(true);
        self.ctime_column.set_resizable(true);
        self.mtime_column.set_resizable(true);
        self.responsible_column.set_resizable(true);

        // Attach cell to column.
        self.id_column.pack_start(&self.id_cell, true);
        self.completed_column.pack_start(&self.completed_cell, true);
        self.name_column.pack_start(&self.name_cell, true);
        self.author_column.pack_start(&self.author_cell, true);
        self.ctime_column.pack_start(&self.ctime_cell, true);
        self.mtime_column.pack_start(&self.mtime_cell, true);
        self.responsible_column.pack_start(&self.responsible_cell, true);

        // Add attribute.
        self.id_column.add_attribute(&self.id_cell, "text", 0);
        self.completed_column.add_attribute(&self.completed_cell, "active", 1);
        self.name_column.add_attribute(&self.name_cell, "text", 2);
        self.author_column.add_attribute(&self.author_cell, "text", 3);
        self.ctime_column.add_attribute(&self.ctime_cell, "text", 4);
        self.mtime_column.add_attribute(&self.mtime_cell, "text", 5);
        self.responsible_column.add_attribute(&self.responsible_cell, "text", 6);

        // Set clickable.
        self.id_column.set_clickable(true);
        self.completed_column.set_clickable(true);
        self.name_column.set_clickable(true);
        self.author_column.set_clickable(true);
        self.ctime_column.set_clickable(true);
        self.mtime_column.set_clickable(true);
        self.responsible_column.set_clickable(true);

        // Set reorderable.
        self.id_column.set_reorderable(true);
        self.completed_column.set_reorderable(true);
        self.name_column.set_reorderable(true);
        self.author_column.set_reorderable(true);

        self.ctime_column.set_reorderable(true);
        self.mtime_column.set_reorderable(true);
        self.responsible_column.set_reorderable(true);

        // Set sort column.
        self.id_column.set_sort_column_id(0);
        self.completed_column.set_sort_column_id(1);
        self.name_column.set_sort_column_id(2);
        self.author_column.set_sort_column_id(3);
        self.ctime_column.set_sort_column_id(4);
        self.mtime_column.set_sort_column_id(5);
        self.responsible_column.set_sort_column_id(6);

        self.tickets_tree_view.append_column(&self.id_column);
        self.tickets_tree_view.append_column(&self.completed_column);
        self.tickets_tree_view.append_column(&self.name_column);
        self.tickets_tree_view.append_column(&self.author_column);
        self.tickets_tree_view.append_column(&self.ctime_column);
        self.tickets_tree_view.append_column(&self.mtime_column);
        self.tickets_tree_view.append_column(&self.responsible_column);
    }

    fn setup_tickets_list_store(&self) {
        self.tickets_tree_view.set_model(Some(&self.tickets_list_store));
    }

    fn setup_directories_list(&self) {
        self.directories_list.insert(&gtk::Label::new(Some("Inbox")), 0);
        self.directories_list.insert(&gtk::Label::new(Some("Outbox")), 1);
        self.directories_list.insert(&gtk::Label::new(Some("Favorites")), 2);
    }

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
        self.pack_toolbar();
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

    fn pack_toolbar(&self) {
        use gtk::ContainerExt;

        self.toolbar.add(&self.create_ticket_t_button);
        self.toolbar.add(&self.create_directory_t_button);
        self.toolbar.add(&gtk::SeparatorToolItem::new());
        self.toolbar.add(&self.open_ticket_t_button);
        self.toolbar.add(&self.edit_access_t_button);
    }

    fn pack_v_box(&self) {
        use gtk::BoxExt;

        self.v_box.pack_start(&self.menu_bar, false, false, 0);
        self.v_box.pack_start(&self.toolbar, false, false, 0);
        self.v_box.pack_start(&self.h_box, true, true, 0);

        self.pack_h_box();
    }

    fn pack_h_box(&self) {
        use gtk::BoxExt;

        self.h_box.pack_start(&self.dir_cal_box, false, false, 0);
        self.h_box.pack_start(&self.tickets_scrolled_window, true, true, 0);

        self.pack_dir_cal_box();
        self.pack_tickets_scrolled_window();
    }

    fn pack_dir_cal_box(&self) {
        use gtk::BoxExt;

        self.dir_cal_box.pack_start(&self.directories_list, true, true, 0);
        self.dir_cal_box.pack_start(&self.calendar, false, false, 0);
    }

    fn pack_tickets_scrolled_window(&self) {
        use gtk::ContainerExt;

        self.tickets_scrolled_window.add(&self.tickets_tree_view);
    }

    fn pack_window(&self) {
        use gtk::{WidgetExt, ContainerExt};

        self.window.add(&self.v_box);

        self.window.show_all();
    }

    pub fn update_ui(&self) {
        self.update_menu_bar();
        self.update_toolbar();
        self.update_main();
    }

    fn update_menu_bar(&self) {
        use gtk::WidgetExt;

        if self.current_user.borrow().is_root() {
            self.root_menu_item.show();
        } else {
            self.root_menu_item.hide();
        }
    }

    fn update_toolbar(&self) {
        use gtk::WidgetExt;

        if self.current_user.borrow().name() != "" {
            self.toolbar.show_all();
        } else {
            self.toolbar.hide();
        }
    }

    fn update_main(&self) {
        use gtk::WidgetExt;

        if self.current_user.borrow().name() != "" {
            self.h_box.show_all();
        } else {
            self.h_box.hide();
        }
    }
}
