use relm4::{
    RelmApp
};
    
mod models;
mod views;

use crate::models::application_model::ApplicationModel;

fn main() {
    let model = ApplicationModel::new();
    let app = RelmApp::new(model);
    app.run();
}