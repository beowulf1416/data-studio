use log::{
    info
};

// use std::cell::RefCell;

use relm4::*;
use gtk::{
    // prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
    // subclass::widget::CompositeTemplate,
    glib,
    glib::subclass::InitializingObject
};
// use gtk::CompositeTemplate;
// use relm4::gtk::gtk4_macros::CompositeTemplate;

// use glib;
// use glib::subclass::InitializingObject;

// use crate::components::data_sources::DataSources;


#[derive(CompositeTemplate, Default)]
#[template(resource="/org/tomale/ds/main.ui")]
pub struct MainWindow {

        // #[template_child]
        // pub data_sources: TemplateChild<DataSources>
}

#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl WidgetImpl for MainWindow {}

impl WindowImpl for MainWindow {}

impl ApplicationWindowImpl for MainWindow {}


impl ObjectImpl for MainWindow {
    fn constructed(&self, obj: &Self::Type) {
        info!("MainWindow::constructed()");
        
        self.parent_constructed(obj);

        // obj.setup_tasks();
        // obj.setup_callbacks();
        // obj.setup_factory();
        // obj.setup_actions();
    }
}