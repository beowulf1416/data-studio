use crate::models::application::Application;
use crate::views::main_window::MainWindow;

pub struct MainViewModel {
    model: Application,
    view: MainWindow
}