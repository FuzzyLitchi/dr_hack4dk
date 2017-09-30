use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use serde_json;
use chrono::NaiveDate;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadioProgram {
    pub allText: String,
    pub date: NaiveDate,
    pub filename: String,
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RadioProgramWrapper {
    _source: RadioProgram,
}

pub fn radio_programs(path: &Path) -> Vec<RadioProgram> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut radio_programs: Vec<RadioProgram> = Vec::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        match serde_json::from_str::<RadioProgramWrapper>(&line) {
            Ok(value) => radio_programs.push(value._source),
            Err(err) => {
                println!("{}\n", line);
                println!("{:?}\n", err);
                panic!("Failed to read line.");
            },
        };
    }

    radio_programs
}
