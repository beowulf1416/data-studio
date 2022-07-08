pub static CORE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");


#[derive(Debug, Clone, PartialEq)]
pub enum InvocationError {
    InvalidArgumentCount {
        expected: usize,
        found: usize
    },
    Other {
        msg: String
    }
}


impl <S: ToString> From<S> for InvocationError {

    fn from(other: S) -> InvocationError {
        return InvocationError::Other {
            msg: other.to_string()
        };
    }
}


pub trait Function {

    fn call(&self, args: &[f64]) -> Result<f64, InvocationError>;

    fn help(&self) -> Option<&str> { None }
}

pub trait PluginRegistrar {
    fn register_function(&mut self, name: &str, function: Box<dyn Function>);
}

#[derive(Copy, Clone)]
pub struct PluginDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
