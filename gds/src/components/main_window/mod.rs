pub mod main_window;

use log::{
    info,
    debug
};

// use relm4::*;
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

// use crate::components::source::SourceView;


glib::wrapper! {
    pub struct MainWindow(ObjectSubclass<main_window::MainWindow>)
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


impl MainWindow {

    pub fn new(app: &Application) -> Self {
        return Object::new(&[("application", app)]).expect("Failed to create window");
    }

    fn setup_actions(&self) {
        info!("MainWindow::setup_actions()");

        let window = self;

        let action_group_add = SimpleAction::new("group-add", None);
        action_group_add.connect_activate(clone!(@weak window => move |_, _| {
            window.group_add();
        }));
        self.add_action(&action_group_add);

        let action_group_close = SimpleAction::new("group-close", None);
        action_group_close.connect_activate(clone!(@weak window => move |_, _| {
            window.group_close();
        }));
        self.add_action(&action_group_close);

        let action_group_save = SimpleAction::new("group-save", None);
        action_group_save.connect_activate(clone!(@weak window => move |_, _| {
            window.group_save();
        }));
        self.add_action(&action_group_save);

        let action_data_source_add = SimpleAction::new("data-source-add", None);
        action_data_source_add.connect_activate(clone!(@weak window => move |_, _| {
            window.data_source_add();
        }));
        self.add_action(&action_data_source_add);

        let action_data_source_save = SimpleAction::new("data-source-save", None);
        action_data_source_save.connect_activate(clone!(@weak window => move |_, _| {
            window.data_source_save();
        }));
        self.add_action(&action_data_source_save);

        let action_data_source_close = SimpleAction::new("data-source-close", None);
        action_data_source_close.connect_activate(clone!(@weak window => move |_, _| {
            window.data_source_close();
        }));
        self.add_action(&action_data_source_close);

        let action_new_query = SimpleAction::new("query-new", None);
        action_new_query.connect_activate(clone!(@weak window => move |_, _| {
            debug!("win.query-new clicked: {:?}", window);
            // debug!("inner: {:?}", window.template_child(gtk::Notebook::static_type(), "qp"));
            // window.test_datasource_add();
        }));
        self.add_action(&action_new_query);
    }

    fn group_add(&self) {
        debug!("MainWindow::group_add()");

        let mw = main_window::MainWindow::from_instance(self);

        let context_id = mw.status.context_id("data_group");
        mw.status.push(context_id, "New data group source");

        mw.stack.set_visible_child_name("group");
    }

    fn group_close(&self) {
        debug!("MainWindow::group_close()");

        let mw = main_window::MainWindow::from_instance(self);

        let context_id = mw.status.context_id("data_group");
        mw.status.pop(context_id);

        mw.stack.set_visible_child_name("panes");
    }

    fn group_save(&self) {
        debug!("MainWindow::group_save()");
        let mw = main_window::MainWindow::from_instance(self);

        let context_id = mw.status.context_id("data_group");
        mw.status.pop(context_id);

        let group = mw.group.get_group_config();
        debug!("group config: {:?}", group);

        mw.stack.set_visible_child_name("panes");
    }

    fn data_source_add(&self){
        debug!("MainWindow::data_source_add()");

        let mw = main_window::MainWindow::from_instance(self);

        let context_id = mw.status.context_id("data_source");
        mw.status.push(context_id, "New data source");

        mw.stack.set_visible_child_name("sources");
    }

    fn data_source_save(&self){
        debug!("MainWindow::data_source_save()");

        let mw = main_window::MainWindow::from_instance(self);
        
        // debug!("selected source type: {:?}", mw.source);
        // mw.source.setup_actions();
        debug!("source config: {:?}", mw.source.get_source_config());

        let context_id = mw.status.context_id("data_source");
        mw.status.push(context_id, "Data source added");

        mw.stack.set_visible_child_name("panes");
    }

    fn data_source_close(&self) {
        debug!("MainWindow::data_source_close()");

        let mw = main_window::MainWindow::from_instance(self);

        let context_id = mw.status.context_id("data_source");
        mw.status.pop(context_id);

        mw.stack.set_visible_child_name("panes");
    }
}