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

#[derive(Debug)]
pub struct SourceConfig {
    pub source_type: String,
    pub name: String,
    pub host: String,
    pub user: String,
    pub password: String,
    pub remember: bool,
    pub additional: String,
}

impl SourceView {

    pub fn new() -> Self {
        return Object::new(&[])
            .expect("Failed to create SourceView");
    }

    pub fn setup_actions(&self) {
        info!("SourceView::setup_actions()");

        // let window = crate::components::main_window::MainWindow::from_instance(self.parent());

        // let action_test = SimpleAction::new("action-test", None);
        // action_test.connect_activate(clone!(@weak window => move |_, _| {
        //     debug!("action_test called");
        // }));
        // window.add_action(&action_test);
    }

    // pub fn get_source_type(&self) {
    //     let sv = source::SourceView::from_instance(self);

    //     debug!("selected source type: {:?}", sv.source_type.active_id());
    // }

    pub fn get_source_config(&self) -> SourceConfig {

        let sv = source::SourceView::from_instance(self);

        let mut type_id = String::from("");
        if let Some(id) = sv.source_type.active_id() {
            type_id = String::from(id.as_str());
        }

        return SourceConfig { 
            source_type: String::from(type_id), 
            name: String::from(sv.source_name.text().as_str()),
            host: String::from(sv.source_name.text().as_str()),
            user: String::from(sv.user_name.text().as_str()),
            password: String::from(sv.password.text().as_str()),
            remember: sv.remember.is_active(),
            additional: String::from(sv.additional.text().as_str())
        };
    }
}