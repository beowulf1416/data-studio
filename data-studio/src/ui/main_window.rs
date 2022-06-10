use std::cell::RefCell;

use gtk4::{
    prelude::*,
    subclass::*,
    CompositeTemplate,
    gio,
    glib::{
        object_subclass,
        subclass::InitializingObject
    }
};


#[derive(CompositeTemplate)]
#[template(resource="../../resources/main.ui")]
pub struct Window {

}

#[object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "MainWindow";
    type Type = super::Window;
    type ParentType = gtk4::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}