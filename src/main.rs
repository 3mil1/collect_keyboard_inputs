extern crate core;

use std::error::Error;
use std::fs;
use std::fs::{File};
use std::io::{Read, Write};
use rdev::{listen, Event};
use serde_json::{Value, Map};


const FILENAME: &str = "key_stat.json";

fn main() {
    let parsed = read_file(FILENAME);
    let mut obj: Map<String, Value> = parsed.unwrap().as_object().unwrap().clone();

    let callback = move |event: Event| {
        if event.name.is_some() {
            let key = event.name.unwrap();
            if obj.contains_key(key.as_str()) {
                let from_json = obj.get(key.as_str()).unwrap();
                let to_int: u32 = from_json.to_string().parse().unwrap();

                obj.insert(key, Value::from(to_int + 1));
            } else {
                obj.insert(key, Value::from(1));
            }


            fs::write(
                FILENAME,
                serde_json::to_string_pretty(&obj).unwrap(),
            ).expect("Unable write to file");
            // println!("{:?}", obj);
        };
    };

    // This will block.
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

pub fn read_file(path: &str) -> Result<Value, Box<dyn Error>> {
    let mut contents = String::new();

    let mut file = File::open(path);
    if file.is_err() {
        file = File::create(path);
        file.as_ref().unwrap().write("{\"a\": 0}".as_bytes()).expect("TODO: panic message");
    }
    let file = File::open(path);
    file.as_ref().unwrap().read_to_string(&mut contents)?;
    let parsed: Value = serde_json::from_str(&contents).expect("Can not parse file");
    Ok(parsed)
}