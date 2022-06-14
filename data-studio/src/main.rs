mod components;

extern crate log;

use log::{
    info,
    error
};

use gtk::prelude::*;
use gtk::{
    gio,
    Application, 
    ApplicationWindow,
    Builder
};





use ui::MainWindow;

const APP_ID: &str = "org.tomale.ds";


fn main() {
    env_logger::init();

    gio::resources_register_include!("ds.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {

    // let window_src = include_str!("../resources/main.ui");
    // let builder = Builder::from_string(window_src);

    // let window = ApplicationWindow::builder()
    //     .application(app)
    //     .title("My GTK App")
    //     .build();

    // let window: ApplicationWindow = builder.object("window.main").unwrap();
    // window.set_application(Some(app));

    let window = MainWindow::new(app);

    // Present window
    window.present();
}
