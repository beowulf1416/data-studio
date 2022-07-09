pub mod group;

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
    },
    subclass::prelude::ObjectSubclassExt
};


glib::wrapper! {
    pub struct GroupView(ObjectSubclass<group::GroupView>)
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


impl Default for GroupView {
    fn default() -> Self {
        return Self::new();
    }
}

#[derive(Debug)]
pub struct GroupConfig {
    pub name: String,
}

impl GroupView {

    pub fn new() -> Self {
        return Object::new(&[])
            .expect("Failed to create SourceView");
    }

    pub fn setup_actions(&self) {
        info!("GroupView::setup_actions()");
    }

    pub fn get_group_config(&self) -> GroupConfig {
        let gv = group::GroupView::from_instance(self);

        return GroupConfig { 
            name: String::from(gv.group_name.text().as_str())
        };
    }
}