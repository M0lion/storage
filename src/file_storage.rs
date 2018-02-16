use std::fs::{OpenOptions,File};
use std::collections::HashMap;
use std::vec::Vec;

use ::Storage;
use ::Error;

pub struct FileStorage {
    file: File,
    data: HashMap<String, Box<[u8]>>
}

impl FileStorage {
    pub fn new(filename: &str) -> FileStorage {
        FileStorage {
            file: OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .append(true)
                .open(filename).unwrap(),
            data: HashMap::new()
        }
    }
}

impl Storage for FileStorage {
    fn read(&self, location: &String) -> Result<Box<[u8]>,Error> {
        match self.data.get(location) {
            Some(d) => Ok(d.clone()),
            None => Err(Error {
                msg: String::from(format!("Could not find any data at location \"{}\"", location))
            })
        }
    }
    fn write(&mut self, location: &String, data: &[u8]) -> Result<(),Error> {
        //write to file

        match self.data.insert(String::from(location), data.into()) {
            Some(d) => false,
            None => true
        }
    }
}