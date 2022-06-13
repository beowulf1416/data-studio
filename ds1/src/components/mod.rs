use log::{
    info
};

use gtk::prelude::{
    BoxExt, 
    ButtonExt, 
    DialogExt, 
    GtkWindowExt, 
    ToggleButtonExt, 
    WidgetExt
};

use relm4::{
    gtk,
    gtk::Builder,
    send, 
    AppUpdate, 
    ComponentUpdate, 
    Model, 
    RelmApp, 
    RelmComponent, 
    Sender, 
    Widgets,
};


enum ApplicationMessage {
    Quit
}

pub struct ApplicationModel {
    window: gtk::ApplicationWindow
}


#[derive(relm4::Components)]
struct ApplicationComponents {
    
}

impl Model for ApplicationModel {
    type Msg = ApplicationMessage;
    type Widgets = ApplicationWidgets;
    type Components = ApplicationComponents;
}


impl Widgets<ApplicationModel, ()> for ApplicationWidgets {

    fn init_view(
        model: &ApplicationModel,
        components: &ApplicationModel::Components,
        sender: Sender<ApplicationModel::Msg>
    ) -> Self {
        let ui_src = include_str!("../../resources/main.ui");
        let builder = Builder::from_string(ui_src);
        let window = builder.object("window.main").unwrap();
        
        return Self {
            window: window
        };
    }
}

