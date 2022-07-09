use log::{
    info
};

use gtk::{
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
    glib,
    // glib::subclass::InitializingObject
};


#[derive(CompositeTemplate, Default)]
#[template(resource="/org/tomale/ds/view.group.ui")]
pub struct GroupView {
    #[template_child]
    pub group_name: TemplateChild<gtk::Entry>
}


#[glib::object_subclass]
impl ObjectSubclass for GroupView {
    const NAME: &'static str = "GroupView";
    type Type = super::GroupView;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}


impl WidgetImpl for GroupView {}

impl BoxImpl for GroupView {}


impl ObjectImpl for GroupView {
    fn constructed(&self, obj: &Self::Type) {
        info!("GroupView::constructed()");
        
        self.parent_constructed(obj);
    }
}