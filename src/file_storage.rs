use std::fs::{OpenOptions,File};
use std::collections::HashMap;
use std::vec::Vec;

use ::Storage;

pub struct FileStorage {
    file: File,
    data: HashMap<String, Box<[u8]>>
}

impl Storage for FileStorage {
    fn read<T: From<Box<[u8]>>>(&self, location: &str) -> Option<T> {
        match self.data.get(&String::from(location)) {
            Some(d) => Some(From::<Box<[u8]>>::from(d.clone())),
            None => None
        }
    }
    fn write<T: From<Box<[u8]>>>(&mut self, location: &str, data: T) -> bool where Box<[u8]>: From<T> {
        //write to file

        match self.data.insert(String::from(location), data.into()) {
            Some(d) => false,
            None => true
        }
    }
}