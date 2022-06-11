mod main_window;

use gtk::{
    prelude::*,
    subclass::prelude::*,
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
        ActionMap
    },
    glib,
    glib::{
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

    pub fn new(app: &Application) -> Self {
        return Object::new(&[("application", app)]).expect("Failed to create main window");
    }
}