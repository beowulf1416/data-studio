pub mod sources;

use log::{
    info,
    debug
};

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
    pub struct DataSourcesView(ObjectSubclass<sources::DataSourcesView>)
        @extends
            gtk::Box,
            gtk::Widget,
        @implements
            gtk::Accessible,
            gtk::Actionable,
            gtk::Buildable,
            gtk::ConstraintTarget
        ;
}


impl Default for DataSourcesView {
    fn default() -> Self {
        return Self::new();
    }
}


impl DataSourcesView {

    pub fn new() -> Self {
        return Object::new(&[])
            .expect("Failed to create DataSourcesView");
    }

    pub fn setup_actions(&self) {
        debug!("DataSourcesView::actions()");

        // let action_data_source_add = SimpleAction::new("data-source-add");
    }
}