use std::fmt::Debug;
use std::path::Path;

use binary2struct::{read_from_file, write_to_file};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Person {
    name: String,
    age: u8,
}
fn main() {
    let person = Person {
        name: "John".to_string(),
        age: 30,
    };
    let file_path = Path::new("person.bin");
    write_to_file(person, file_path);
    read_from_file::<Person>(file_path);
}
