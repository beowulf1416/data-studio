mod components;

extern crate log;

use relm4::{
    RelmApp
};

use crate::components::ApplicationModel;

fn main() {
    let model = ApplicationModel {
        // mode: AppMode::View,
    };
    let relm = RelmApp::new(model);
    relm.run();
}
