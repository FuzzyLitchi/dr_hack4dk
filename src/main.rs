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
mod searching;

use tantivy::collector::TopCollector;
use searching::Searcher;

fn main() {
    let searcher = Searcher::default();

    let query = searcher.query_parser.parse_query("Hej mor").unwrap();

    let mut top_collector = TopCollector::with_limit(10);

    searcher.index.searcher().search(&*query, &mut top_collector).unwrap();
    let doc_addresses = top_collector.docs();

    for doc_address in doc_addresses {
       let retrieved_doc = searcher.index.searcher().doc(&doc_address).unwrap();
       println!("{}", searcher.index.schema().to_json(&retrieved_doc));
    }
}
