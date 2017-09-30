#[macro_use]
extern crate serde_derive;

#[allow(unused_extern_crates)]
extern crate serde;
extern crate serde_json;
extern crate chrono;
#[macro_use]
extern crate tantivy;
extern crate tempdir;

mod data;

use std::path::Path;
use tempdir::TempDir;
use tantivy::Index;
use tantivy::schema::*;
use tantivy::collector::TopCollector;
use tantivy::query::QueryParser;

use data::radio_programs;

fn main() {
    let index_dir = TempDir::new("dr_index").unwrap();
    let mut schema_builder = SchemaBuilder::default();

    //The searchable items
    let title = schema_builder.add_text_field("title", TEXT | STORED);
    let all_text = schema_builder.add_text_field("allText", TEXT | STORED);

    let schema = schema_builder.build();
    let index = Index::create(index_dir.path(), schema.clone()).unwrap();

    let mut index_writer = index.writer(50_000_000).unwrap();

    for v in radio_programs(Path::new("programoversigter.json")) {
        index_writer.add_document(doc!(
            title => v.title,
            all_text => v.allText
        ));
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
