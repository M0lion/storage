use std::fs::{OpenOptions,File};
use std::io::{Seek,SeekFrom,Write,Read};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
use ::Error;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use ::std::vec::Vec;
use ::std::str;

pub struct HashFile {
    file: File,
    map_size: u64,
}

struct Node {
    string: String,
    dataAddr: u64,
    data: u64,
    next: u64
}

impl Node {
    fn read(file: &mut File, addr: u64) -> Result<Node, ::std::io::Error> {
        file.seek(SeekFrom::Start(addr)).unwrap();
        let size = file.read_u8()?;

        let mut buff = Vec::new();
        {
            let mut chunk = file.take(u64::from(size));
            chunk.read_to_end(&mut buff)?;
        }
        let string = String::from(str::from_utf8(&buff).unwrap());

        let data = file.read_u64::<BigEndian>()?;

        let next = file.read_u64::<BigEndian>()?;
        
        Ok(Node{
            string,
            dataAddr: addr + 1 + u64::from(size),
            data,
            next
        })
    }
}

impl HashFile {
    pub fn load(filename: &str) -> Result<HashFile,::std::io::Error> {
        match OpenOptions::new().create(false).write(true).read(true).append(true).open(filename) {
            Ok(mut file) => {
                file.seek(SeekFrom::Start(0))?;
                let map_size = file.read_u64::<BigEndian>().unwrap();
                Ok(HashFile {
                    file,
                    map_size,
                })
            },
            Err(e) => Err(e)
        }
    }

    pub fn create(filename: &str, map_size: u64) -> Result<HashFile,::std::io::Error> {
        match OpenOptions::new().create(true).write(true).read(true).append(false).open(filename) {
            Ok(mut file) => {
                file.write_u64::<BigEndian>(map_size)?;
                for _i in 0..map_size {
                    file.write_u64::<BigEndian>(0)?;
                }
                file.flush()?;

                Ok(HashFile {
                    file,
                    map_size,
                })
            },
            Err(e) => Err(e)
        }
    }

    fn get_node(&mut self, location: &str) -> Result<Node,()> {
        let mut hasher = DefaultHasher::new();
        String::from(location).hash(&mut hasher);
        let hash: u64 = hasher.finish();
        let file = &mut self.file;
        file.seek(SeekFrom::Start(hash + 8)).unwrap();
        let addr = file.read_u64::<BigEndian>().unwrap();

        let mut node = Node::read(file, addr).unwrap();

        let loc = String::from(location);

        loop {
            if node.string.eq(&loc) {
                return Ok(node);
            }

            if node.next == 0 {
                break;
            }

            node = Node::read(file, node.next).unwrap();
        }

        Err(())
    }

    pub fn get(&mut self, location: &str) -> Result<u64,Error> {
        match self.get_node(location) {
            Ok(node) => Ok(node.data),
            Err(()) => Err(Error {
                msg: String::from(format!("could not find {} in HashFile", location))
            })
        }
    }

    pub fn set(&mut self, data: u64, location: &str) -> Result<(),Error> {
        match self.get_node(location) {
            Ok(node) => {
                self.file.seek(SeekFrom::Start(node.dataAddr)).unwrap();
                self.file.write_u64::<BigEndian>(data).unwrap();
            },
            Err(()) => {
                //new node
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ::hash_file::HashFile;

    #[test]
    fn test() {
        let hashFile = HashFile::create("Test.strg", 63);
    }   
}