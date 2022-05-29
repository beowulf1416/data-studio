use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("resources.gresource");

    let args = [
        format!("--target={}", out_path.display()),
        "resources/resources.gresource.xml".to_string(),
    ];

    let output = Command::new("glib-compile-resources")
        .args(&args)
        .output()
        .unwrap();
    println!("{}", output.status);
    

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=resources/resources.gresource.xml");

    println!("cargo:rerun-if-changed=resources/ui/main.glade");
    println!("cargo:rerun-if-changed=resources/ui/editor.sql.glade");
    println!("cargo:rerun-if-changed=resources/ui/connection-proviers.glade");
}