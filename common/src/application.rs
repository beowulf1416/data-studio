use std::{ env, fs };

use crate::providers::registry::{ Registry };

pub struct Application {
    registry: Registry,
}

impl Application {

    pub fn new() -> Self {
        println!("Application::new()");

        let application = Self {
            registry: Registry::new()
        };
        
        return application;
    }

    pub fn providers(self) -> Vec<String> {
        let current_dir = env::current_dir();
        println!("{:?}", current_dir);

        let current_exe = env::current_exe().unwrap();
        let mut path = current_exe;
        path.pop();
        println!("current path: {:?}", path);

        let mut providers = Vec::new();

        for entry in fs::read_dir(path).unwrap() {
            if let Ok(f) = entry {
                if let Ok(ftype) = f.file_type() {
                    if ftype.is_file() {
                        let fname = f.file_name().into_string().unwrap();
                        // check if file name matches with libplugin_*.sq
                        if fname.starts_with("libplugin_") && fname.ends_with(".so") {
                            providers.push(fname);
                        }                        
                    }
                }
            }
            
        }

        return providers;
    }
}