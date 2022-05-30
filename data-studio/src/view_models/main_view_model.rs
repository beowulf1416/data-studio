use gtk::{
    Window
};
use relm::{ Widget };

use crate::models::application::Application;
use crate::views::main_window::MainWindow;

pub struct MainViewModel {
    model: Application,
    view: MainWindow
}

impl Widget for MainViewModel {
    type Root = Window;

    fn root(&self) -> Self::Root {
        return self.view.MainWindow.clone();
    }    

}