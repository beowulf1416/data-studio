use gio;

fn main() {
    gio::compile_resources(
        "resources",
        "resources/gds.gresource.xml",
        "gds.gresource",
    );
}