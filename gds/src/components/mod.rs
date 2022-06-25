mod main_window;

use log::{
    info
};

use gtk::prelude::{ WidgetExt };
use relm4::*;

use crate::components::main_window::MainWindow;


pub enum ApplicationMessages {
    Exit
}


pub struct ApplicationWidgets {
    window: MainWindow
}


pub struct Application {
    app: gtk::Application
}


pub struct ApplicationComponents {

}


impl Application {

    pub fn new(app: gtk::Application) -> Self {
        return Self {
            app: app
        };
    }
}


impl Model for Application {
    type Msg = ApplicationMessages;
    type Widgets = ApplicationWidgets;
    type Components = ApplicationComponents;
}


impl AppUpdate for Application {

    fn update(
        &mut self,
        msg: ApplicationMessages,
        components: &ApplicationComponents,
        sender: Sender<ApplicationMessages>
    ) -> bool {
        match msg {
            ApplicationMessages::Exit => {
                info!("Exit called");
            }
        }
        return true;
    }
}



impl Widgets<Application, ()> for ApplicationWidgets {
    // type Root = gtk::ApplicationWindow;
    type Root = MainWindow;

    fn init_view(
        model: &Application,
        components: &ApplicationComponents,
        sender: Sender<ApplicationMessages>
    ) -> Self {
        let window = MainWindow::new(&model.app);
        

        return ApplicationWidgets {
            window: window
        }
    }

    fn root_widget(&self) -> Self::Root {
        return self.window.clone();
    }

    fn view(
        &mut self,
        model: &Application,
        sender: Sender<ApplicationMessages>
    ) {
        info!("ApplicationWidgets::view()");
    }

    fn connect_parent(
        &mut self,
        _widgets: &()
    ) {
        info!("ApplicationWidgets::connect_parent()");
    }
}







impl Components<Application> for ApplicationComponents {

    fn init_components(
        parent: &Application,
        // widgets: &ApplicationWidgets,
        sender: Sender<ApplicationMessages>
    ) -> Self {
        return ApplicationComponents {

        };
    }

    fn connect_parent(
        &mut self,
        widgets: &<crate::components::Application as Model>::Widgets
    ) {
        info!("ApplicationComponents::connect_parent()");
    }
}