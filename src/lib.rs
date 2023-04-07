use std::fmt::Debug;
use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Read, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};

pub fn write_to_file<'a, T>(obj: T, file_path: &Path)
where
    T: Serialize + Deserialize<'a> + PartialEq + Debug,
{
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
    {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(file_path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    file.set_len(0).unwrap();
    let encode_binary: Vec<u8> = bincode::serialize(&obj).unwrap();
    file.write_all(&encode_binary).unwrap();
}
pub fn read_from_file<'a, T>(file_path: &'a Path) -> T
where
    T: Serialize + for<'de> Deserialize<'de> + PartialEq + Debug,
{
    let mut file = File::open(file_path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let decode_binary: T = bincode::deserialize(&buffer).unwrap();
    decode_binary
}
#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Person {
        name: String,
        age: u8,
    }
    #[serial]
    fn test_write_to_file() {
        let person = Person {
            name: "John".to_string(),
            age: 30,
        };
        let file_path = Path::new("person.bin");
        write_to_file(person, file_path);
        read_from_file::<Person>(file_path);
    }

    #[test]
    #[serial]
    fn test_read_from_file() {
        let file_path = Path::new("person.bin");
        let person = read_from_file::<Person>(file_path);
        println!("{:?}", person);
    }
}
