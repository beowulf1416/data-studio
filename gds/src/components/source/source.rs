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
#[template(resource="/org/tomale/ds/view.source.ui")]
pub struct SourceView {
    // #[template_child]
    // pub tv: TemplateChild<gtk::TreeView>
}


#[glib::object_subclass]
impl ObjectSubclass for SourceView {
    const NAME: &'static str = "SourceView";
    type Type = super::SourceView;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}


impl WidgetImpl for SourceView {}

impl BoxImpl for SourceView {}


impl ObjectImpl for SourceView {
    fn constructed(&self, obj: &Self::Type) {
        info!("SourceView::constructed()");
        
        self.parent_constructed(obj);

        // obj.setup_tasks();
        // obj.setup_callbacks();
        // obj.setup_factory();
        obj.setup_actions();
    }
}