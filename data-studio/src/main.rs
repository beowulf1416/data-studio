mod models;
mod views;

use relm::{ Widget };


use crate::views::main_window::MainWindow;


fn main() {
    MainWindow::run(()).expect("MainWindow::run() failed");
}