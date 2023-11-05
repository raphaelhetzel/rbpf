#[derive(core::fmt::Debug)]
pub struct Error {
    message: alloc::string::String
}

impl Error {
    pub fn new(message: alloc::string::String) -> Self {
        Error {
            message: message
        }
    }
}