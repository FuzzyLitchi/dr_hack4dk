#[macro_use]
extern crate serde_derive;

#[allow(unused_extern_crates)]
extern crate serde;
extern crate serde_json;
extern crate chrono;
extern crate tantivy;
extern crate tempdir;

use chrono::NaiveDate;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use tempdir::TempDir;
use tantivy::Index;
use tantivy::schema::*;
use tantivy::collector::TopCollector;
use tantivy::query::QueryParser;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
struct RadioProgram {
    allText: String,
    date: NaiveDate,
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

    let index_dir = TempDir::new("dr_index").unwrap();
    let index_path = index_dir.path();
    let mut schema_builder = SchemaBuilder::default();

    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let all_text = schema_builder.add_text_field("allText", TEXT | STORED);

    let schema = schema_builder.build();
    let index = Index::create(index_path, schema.clone()).unwrap();

    let mut index_writer = index.writer(50_000_000).unwrap();
    for v in values {
        let mut document = Document::default();
        document.add_text(title, &v.title);
        document.add_text(all_text, &v.allText);

        index_writer.add_document(document);
    }

    index_writer.commit().unwrap();
    index.load_searchers().unwrap();
    let searcher = index.searcher();

    let query_parser = QueryParser::new(index.schema(), vec![title, all_text]);

    let query = query_parser.parse_query("Hej mor").unwrap();

    let mut top_collector = TopCollector::with_limit(10);

    searcher.search(&*query, &mut top_collector).unwrap();
    let doc_addresses = top_collector.docs();

    for doc_address in doc_addresses {
       let retrieved_doc = searcher.doc(&doc_address).unwrap();
       println!("{}", schema.to_json(&retrieved_doc));
    }
}
