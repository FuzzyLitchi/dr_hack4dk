use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use serde_json;
use tantivy::Document;
use tantivy::schema::Value;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadioProgram {
    pub allText: String,
    pub date: String,
    pub filename: String,
    pub title: String,
    pub url: String,
}

impl RadioProgram {
    pub fn from_document(doc: Document) -> RadioProgram {
        let mut values: Vec<String> = doc.field_values().iter().map(|field_value| {
            match field_value.value() {
                &Value::Str(ref string) => string.clone(),
                _ => panic!("I am tired")
            }
        }).collect();

        //I know this is bad and you're allowed to murder me ty
        RadioProgram {
            allText: values.remove(0),
            date: values.remove(0),
            filename: values.remove(0),
            title: values.remove(0),
            url: values.remove(0),
        }
    }
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
