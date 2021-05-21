use gio::prelude::*;
use gtk::prelude::*;
// use gdk::prelude::*;

// use gtk_macros::{ action };
use crate::constants::{ PROVIDERS_UI };

pub struct ProviderDialog {

}

impl ProviderDialog {
    
    pub fn show(parent: &gtk::Window) {
        let builder = gtk::Builder::from_resource(PROVIDERS_UI);
        let dialog: gtk::Dialog = builder.get_object("window.connection.providers").unwrap();
        // dialog.set_type_hint();
        dialog.set_transient_for(Some(parent));
        dialog.set_modal(true);
        dialog.run();
    }
}