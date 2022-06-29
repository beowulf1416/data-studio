pub mod main_window;

use log::{
    info,
    debug
};

// use relm4::*;
use gtk::{
    prelude::*,
    // subclass::prelude::*,
    Accessible,
    Application,
    ApplicationWindow,
    Buildable,
    ConstraintTarget,
    Native,
    Root,
    ShortcutManager,
    Window,
    Widget,
    gio::{
        ActionGroup,
        ActionMap,
        SimpleAction
    },
    glib,
    glib::{
        clone,
        Object
    }
};


glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<main_window::MainWindow>)
        @extends 
            ApplicationWindow,
            Window,
            Widget,
        @implements
            ActionGroup,
            ActionMap,
            Accessible,
            Buildable,
            ConstraintTarget,
            Native,
            Root,
            ShortcutManager;
}


impl MainWindow {

    // pub fn new() -> Self {
    //     // return Object::new(&[("application", app)]).expect("Failed to create main window");
    //     return Object::new(&[]).expect("Failed to create main window");
    // }

    pub fn new(app: &Application) -> Self {
        return Object::new(&[("application", app)]).expect("Failed to create window");
    }

    fn setup_actions(&self) {
        info!("MainWindow::setup_actions()");

        let window = self;

        let action_new_data_source = SimpleAction::new("data-source-add", None);
        action_new_data_source.connect_activate(clone!(@weak window => move |_, _| {
            debug!("win.new.data-source clicked: {:?}", window);
        }));
        self.add_action(&action_new_data_source);

        let action_new_query = SimpleAction::new("query-new", None);
        action_new_query.connect_activate(clone!(@weak window => move |_, _| {
            debug!("win.query-new clicked: {:?}", window);
            // debug!("inner: {:?}", window.template_child(gtk::Notebook::static_type(), "qp"));
        }));
        self.add_action(&action_new_query);
    }
}