extern crate byteorder;

pub mod hash_file;

pub trait Storage {
    fn read(&self, location: &String) -> Result<Box<[u8]>,std::error::Error>;
    fn write(&mut self, location: &String, data: &[u8]) -> Result<(),Error>;
}

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Storage Error: {}", self.msg)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.msg.as_str()
    }
}