pub mod sources;
pub mod tree;

use log::{
    info,
    debug
};

use gtk::{
    // prelude::*,
    // // subclass::prelude::*,
    // Accessible,
    // Application,
    // ApplicationWindow,
    // Buildable,
    // ConstraintTarget,
    // Native,
    // Root,
    // ShortcutManager,
    // Window,
    // Widget,
    // gio::{
    //     ActionGroup,
    //     ActionMap,
    //     SimpleAction
    // },
    glib,
    glib::{
        // clone,
        Object
    }, 
    subclass::prelude::ObjectSubclassExt
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

    pub fn group_add(&self, name: String) {
        debug!("DataSourcesView::group_add()");

        let ds = sources::DataSourcesView::from_instance(self);

        // ds.tv.model().insert_with_values(
        //     None,
        //     None,
        //     &[
        //         (0, name)
        //     ]
        // );
    }
}