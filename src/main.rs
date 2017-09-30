#[macro_use]
extern crate serde_derive;

extern crate serde_json;
#[macro_use]
extern crate tantivy;
extern crate tempdir;

mod data;
mod searching;

use searching::Searcher;

fn main() {
    let searcher = Searcher::default();
    println!("Searcher is ready!");

    for doc_address in searcher.search("Hello", 10) {
       let retrieved_doc = searcher.index.searcher().doc(&doc_address).unwrap();
       println!("{}", searcher.index.schema().to_json(&retrieved_doc));
    }
}
