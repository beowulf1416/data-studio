pub mod source;

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
    pub struct SourceView(ObjectSubclass<source::SourceView>)
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


impl Default for SourceView {
    fn default() -> Self {
        return Self::new();
    }
}


impl SourceView {

    pub fn new() -> Self {
        return Object::new(&[])
            .expect("Failed to create SourceView");
    }

    fn setup_actions(&self) {
        info!("SourceView::setup_actions()");

        // let window = crate::components::main_window::MainWindow::from_instance(self.parent());

        // let action_test = SimpleAction::new("action-test", None);
        // action_test.connect_activate(clone!(@weak window => move |_, _| {
        //     debug!("action_test called");
        // }));
        // window.add_action(&action_test);
    }
}