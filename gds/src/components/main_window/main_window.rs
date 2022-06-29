use log::{
    info
};

// use relm4::*;
use gtk::{
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
    glib,
    glib::subclass::InitializingObject
};

use crate::components::sources::DataSourcesView;



#[derive(CompositeTemplate, Default)]
#[template(resource="/org/tomale/ds/main.ui")]
pub struct MainWindow {

        // #[template_child]
        // pub data_sources: TemplateChild<DataSources>

        #[template_child]
        pub dsv: TemplateChild<DataSourcesView>,

        #[template_child]
        pub qp: TemplateChild<gtk::Notebook>,
}


#[glib::object_subclass]
impl ObjectSubclass for MainWindow {
    const NAME: &'static str = "MainWindow";
    type Type = super::MainWindow;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        info!("MainWindow::class_init()");

        DataSourcesView::ensure_type();

        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        info!("MainWindow::instance_init()");
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
        obj.setup_actions();
    }
}