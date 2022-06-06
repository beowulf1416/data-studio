use gtk::{
    prelude::BuilderExtManual,
    Builder,
    Window
};

use relm_derive::Msg;
use relm::{connect, Relm, Update, Widget, WidgetTest};

use crate::models::application::Application;


#[derive(Clone)]
pub struct MainWindow {
    application: Application,
    window: Window
}

impl Widget for MainWindow {

    type Root = Window;

    fn root(&self) -> Self::Root {
        return self.window.clone();
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let glade_src = include_str!("../../resources/main.glade");
        let builder = Builder::from_string(glade_src);

        let window: Window = builder.object("main.window").unwrap();
        window.show_all();

        return MainWindow {
            application: model,
            window: window
        };
    }
}


impl Update for MainWindow {

    type Model = Application;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        return Application {};
    }

    fn update(&mut self, event: Msg) {

    }
}