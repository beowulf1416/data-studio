use log::{
    info
};

// use std::cell::RefCell;

use gtk::{
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
    glib,
    glib::subclass::InitializingObject
};

#[derive(CompositeTemplate, Default)]
#[template(resource="/org/tomale/ds/data-sources.ui")]
pub struct DataSources {

}

#[glib::object_subclass]
impl ObjectSubclass for DataSources {
    const NAME: &'static str = "DataSources";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}