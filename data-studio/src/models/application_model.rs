use relm4::{
    AppUpdate, 
    Model, 
    Sender
};


use crate::views::main_window::MainWindow;


pub enum ApplicationMessage {
    Quit
}


#[derive(Clone)]
pub struct ApplicationModel {
    
}

impl ApplicationModel {

    pub fn new() -> Self {
        return Self {};
    }
}

impl Model for ApplicationModel {
    type Msg = ApplicationMessage;
    type Widgets = MainWindow;
    type Components = ();
}

impl AppUpdate for ApplicationModel {
    fn update(&mut self, msg: ApplicationMessage, _components: &(), _sender: Sender<ApplicationMessage>) -> bool {
        match msg {
            ApplicationMessage::Quit => {
                // do something
            }
        }

        return true;
    }
}