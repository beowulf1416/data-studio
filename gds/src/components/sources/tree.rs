use log::{
    info,
    debug
};

use gtk::{
    prelude::*,
    glib,
    ffi::*
};
// use gtk::ffi::*;


glib::wrapper! {
    pub struct Tree(Object<GtkTreeStore, GtkTreeStoreClass>)
        @implements
            gtk::TreeModel
    ;

    match fn {
        type_ => || gtk_tree_store_get_type(),
    }
}


impl Tree {


    // pub fn new() -> Tree {

    //     return Self {};
    // }
}

