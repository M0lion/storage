use std::vec::Vec;
use std::convert::{From,Into};

pub mod file_storage;

pub trait Storage {
    fn read<T: From<Box<[u8]>>>(&self, location: &str) -> Option<T>;
    fn write<T: From<Box<[u8]>>>(&mut self, location: &str, data: T) -> bool where Box<[u8]>: From<T>;
}

#[cfg(test)]
mod tests {
    fn testStorage<T: Storage>(s: T){

    }

    #[test]
    fn readW_wite() {
        assert_eq!(2 + 2, 4);
    }   
}