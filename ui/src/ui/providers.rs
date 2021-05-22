use gio::prelude::*;
use gtk::prelude::*;
// use gdk::prelude::*;
use glib::clone;
use gtk_macros::{ action };

// use gtk_macros::{ action };
use crate::constants::{ PROVIDERS_UI };

use common::application::Application;


pub struct ProviderDialog {
    dialog: gtk::Dialog
}

impl ProviderDialog {

    pub fn new(application: &Application) -> Result<Self, Box<dyn std::error::Error>> {
        let builder = gtk::Builder::from_resource(PROVIDERS_UI);
        let dialog: gtk::Dialog = builder.get_object("window.connection.providers").unwrap();
        dialog.set_type_hint(gdk::WindowTypeHint::Dialog);
        dialog.set_modal(true);
        dialog.set_default_response(gtk::ResponseType::Cancel);


        let lbr = gtk::ListBoxRow::new();
        let text = gtk::Label::new(Some("test"));
        lbr.add(&text);

        let lb: gtk::ListBox = builder.get_object("providers.list").expect("no listbox found");
        lb.add(&lbr);

        // println!("{:?}", get_widget!(dialog, gtk::ListBox, providers));
        // println!("{:?}", application.providers());

        // setup actions
        let actions = gio::SimpleActionGroup::new();
        dialog.insert_action_group("actions", Some(&actions));

        action!(actions, "providers.cancel", clone!(@weak dialog => move |_, _| {
                dialog.response(gtk::ResponseType::Cancel);
            }
        ));

        action!(actions, "providers.ok", clone!(@weak dialog => move |_, _| {
                dialog.response(gtk::ResponseType::Ok);
            }
        ));

        dialog.connect_response(clone!(@weak dialog => move |_, response| {
                println!("connect_response");
                println!("{:?}", response);
                dialog.close();
            }
        ));

        let provider_dialog = Self {
            dialog: dialog
        };

        return Ok(provider_dialog);
    }
    
    pub fn show(&self, parent: &gtk::Window) -> gtk::ResponseType {
        self.dialog.set_transient_for(Some(parent));
        return self.dialog.run();
    }
}