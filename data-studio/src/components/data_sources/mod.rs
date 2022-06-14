mod data_sources;

use log::{
    info
};

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
    pub struct DataSources(ObjectSubclass<data_sources::DataSources>)
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


impl DataSources {

    pub fn new(app: &Application) -> Self {
        return Object::new(&[("application", app)]).expect("Failed to create main window");
    }

    fn setup_actions(&self) {
        info!("DataSources::setup_actions()");

        // let window = self;

        // let action_new_data_source = SimpleAction::new("data-source-add", None);
        // action_new_data_source.connect_activate(clone!(@weak window => move |_, _| {
        //     debug!("win.new.data-source clicked: {:?}", window);
        // }));
        // self.add_action(&action_new_data_source);
    }
}