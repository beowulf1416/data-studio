use log::{
    info,
    debug
};

// use relm4::*;
use gtk::{
    prelude::*,
    subclass::prelude::*,
    CompositeTemplate,
    glib,
    glib::{
        clone,
        subclass::InitializingObject
    },
    gio::SimpleAction
};

use crate::components::sources::DataSourcesView;



#[derive(CompositeTemplate, Default)]
#[template(resource="/org/tomale/ds/main.ui")]
pub struct MainWindow {

    // #[template_child]
    // pub data_sources: TemplateChild<DataSources>

    #[template_child]
    pub stack: TemplateChild<gtk::Stack>,

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

        // klass.install_action("win.data-source-add", None, move |win, _, _| {
        //     debug!("win.data-source-add");

        //     // win.test_datasource_add();
        // });
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
        
        // self.setup_actions();
    }
}


impl MainWindow {

    // fn test_datasource_add(&self) {
    //     debug!("MainWindow::test_datasource_add()");
    // }

    pub fn setup_actions(&self) {
        debug!("MainWindow::setup_actions() a");

        // let window = self;

        // let action_new_data_source = SimpleAction::new("data-source-add", None);
        // action_new_data_source.connect_activate(clone!(@weak window => move |_, _| {
        //     debug!("win.new.data-source clicked: {:?}", window);

        // }));
        // self.add_action(&action_new_data_source);

        // let action_new_query = SimpleAction::new("query-new", None);
        // action_new_query.connect_activate(clone!(@weak window => move |_, _| {
        //     debug!("win.query-new clicked: {:?}", window);
        //     // debug!("inner: {:?}", window.template_child(gtk::Notebook::static_type(), "qp"));
        //     // window.test_datasource_add();
        // }));
        // self.add_action(&action_new_query);
    }
}