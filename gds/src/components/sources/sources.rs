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
#[template(resource="/org/tomale/ds/view.data_sources.ui")]
pub struct DataSourcesView {

}


#[glib::object_subclass]
impl ObjectSubclass for DataSourcesView {
    const NAME: &'static str = "DataSourcesView";
    type Type = super::DataSourcesView;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}


// impl ObjectImpl for DataSourcesView {}

impl WidgetImpl for DataSourcesView {}

impl BoxImpl for DataSourcesView {}


impl ObjectImpl for DataSourcesView {
    fn constructed(&self, obj: &Self::Type) {
        info!("DataSourcesView::constructed()");
        
        self.parent_constructed(obj);

        // obj.setup_tasks();
        // obj.setup_callbacks();
        // obj.setup_factory();
        // obj.setup_actions();
    }
}