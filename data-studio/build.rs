use gtk::gio;

fn main() {
    gio::compile_resources(
        "resources",
        "resources/ds.gresource.xml",
        "ds.gresource",
    );
}