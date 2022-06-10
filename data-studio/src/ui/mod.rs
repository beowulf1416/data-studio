mod main_window;

use gtk4::{
    prelude::*,
    subclass::prelude::*,
    Accessible,
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
    glib
};


glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<main_window::Window>)
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