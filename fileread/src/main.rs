use std::fs;
use std::fmt;
use serde_derive::{Serialize, Deserialize};
use serde_json;


#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    id: i32
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.id)
    }
}

fn main() {
    println!("Hello, world!");
    let data = fs::read_to_string("src/input.json").expect("unable to read file");
    let  res:Vec<User> = serde_json::from_str(&data).unwrap();
    println!("count:{}", res.len());
    for item in &res {
        println!("{}", item);
    }
}
