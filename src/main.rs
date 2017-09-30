#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct RadioProgram {
    allText: String,
    date: String,
    filename: String,
    title: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RadioProgramWrapper {
    _source: RadioProgram,
}

fn main() {
    let file = File::open("programoversigter.json").unwrap();
    let reader = BufReader::new(file);

    let mut values: Vec<RadioProgram> = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        match serde_json::from_str::<RadioProgramWrapper>(&line) {
            Ok(value) => values.push(value._source),
            Err(err) => {
                println!("{}\n", line);
                println!("{:?}\n", err);
                panic!("Failed to read line.");
            },
        };
    }
}
