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



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
