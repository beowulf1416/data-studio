use gtk::{
    prelude::BuilderExtManual,
    Builder,
    Inhibit,
    Window,
    prelude::WidgetExt,
    prelude::GtkMenuItemExt,
};

use relm_derive::Msg;
use relm::{connect, Relm, Update, Widget, WidgetTest};

use crate::models::application::Application;


#[derive(Msg)]
pub enum MainWindowMsg {
    Quit
}


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

        let window: Window = builder.object("window.main").unwrap();
        window.show_all();

        let menu_file_new: gtk::MenuItem = builder.object("window.main.menu.file.new").unwrap();
        let menu_file_open: gtk::MenuItem = builder.object("window.main.menu.file.open").unwrap();
        let menu_file_quit: gtk::MenuItem = builder.object("window.main.menu.file.quit").unwrap();

        connect!(relm, window, connect_delete_event(_, _), return (Some(MainWindowMsg::Quit), Inhibit(false)));
        connect!(menu_file_new, connect_activate(_), relm, MainWindowMsg::Quit);
        connect!(menu_file_open, connect_activate(_), relm, MainWindowMsg::Quit);
        connect!(menu_file_quit, connect_activate(_), relm, MainWindowMsg::Quit);

        return MainWindow {
            application: model,
            window: window
        };
    }
}


impl Update for MainWindow {

    type Model = Application;
    type ModelParam = ();
    type Msg = MainWindowMsg;

    fn model(_: &Relm<Self>, _: ()) -> Self::Model {
        return Application {};
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            MainWindowMsg::Quit => {
                gtk::main_quit();
            }
        }
    }
}