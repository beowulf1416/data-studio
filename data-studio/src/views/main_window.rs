use relm4::{
    gtk,
    gtk::Builder,
    Sender, 
    Widgets
};



use crate::models::application_model::{
    ApplicationMessage,
    ApplicationModel
};



pub struct MainWindow {
    window: gtk::ApplicationWindow
}


impl Widgets<ApplicationModel, ()> for MainWindow {
    type Root = gtk::ApplicationWindow;

    fn init_view(
        model: &ApplicationModel, 
        _components: &(), 
        sender: Sender<ApplicationMessage>
    ) -> Self {
        let ui_src = include_str!("../../resources/main.glade");
        let builder = Builder::from_string(ui_src);

        let window: gtk::ApplicationWindow = builder.object("window.main").expect("could not get application window");

        return Self {
            window: window
        };
    }

    fn root_widget(&self) -> Self::Root {
        return self.window.clone();
    }

    fn view(&mut self, model: &ApplicationModel, _sender: Sender<ApplicationMessage>) {
        // do something
    }
}