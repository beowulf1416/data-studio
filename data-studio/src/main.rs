extern crate log;

use log::{
    info,
    error
};

use gtk4::prelude::*;
use gtk4::{
    Application, 
    ApplicationWindow,
    Builder
};


mod ui;


use ui::MainWindow;

fn main() {
    let app = Application::builder()
        .application_id("org.tomale.ds")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window_src = include_str!("../resources/main.ui");
    let builder = Builder::from_string(window_src);

    // let window = ApplicationWindow::builder()
    //     .application(app)
    //     .title("My GTK App")
    //     .build();
    let window: ApplicationWindow = builder.object("window.main").unwrap();
    window.set_application(Some(app));


    // Present window
    window.present();
}
