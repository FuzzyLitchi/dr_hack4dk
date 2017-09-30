use std::path::Path;
use tempdir::TempDir;
use tantivy::Index;
use tantivy::query::QueryParser;
use tantivy::collector::TopCollector;
use tantivy::schema::*;
use data::{RadioProgram, radio_programs};

pub struct Searcher {
    pub index: Index,
    pub index_dir: TempDir,
    pub query_parser: QueryParser,
}

impl Searcher {
    pub fn new(index: Index,
               index_dir: TempDir,
               query_parser: QueryParser
    ) -> Searcher {
        Searcher {
            index,
            index_dir,
            query_parser,
        }
    }

    pub fn default() -> Searcher {
        let index_dir = TempDir::new("dr_index").unwrap();
        let mut schema_builder = SchemaBuilder::default();

        //The searchable items
        let all_text = schema_builder.add_text_field("allText", TEXT | STORED);
        let date = schema_builder.add_text_field("date", TEXT | STORED);
        let filename = schema_builder.add_text_field("filename", TEXT | STORED);
        let title = schema_builder.add_text_field("title", TEXT | STORED);
        let url = schema_builder.add_text_field("url", TEXT | STORED);

        let schema = schema_builder.build();
        let index = Index::create(index_dir.path(), schema.clone()).unwrap();

        let mut index_writer = index.writer(50_000_000).unwrap();

        for v in radio_programs(Path::new("programoversigter.json")) {
            index_writer.add_document(doc!(
                all_text => v.allText,
                date => v.date,
                filename => v.filename,
                title => v.title,
                url => v.url
            ));
        }

        index_writer.commit().unwrap();
        index.load_searchers().unwrap();

        let query_parser = QueryParser::new(schema, vec![title, all_text]);

        Searcher::new(
            index,
            index_dir,
            query_parser,
        )
    }

    pub fn search(&self, query_string: &str, limit: usize) -> Vec<RadioProgram> {
        let query = self.query_parser.parse_query(&query_string).unwrap();

        let mut top_collector = TopCollector::with_limit(limit);

        self.index.searcher().search(&*query, &mut top_collector).unwrap();

        let mut results: Vec<RadioProgram> = Vec::with_capacity(limit);

        for doc_address in top_collector.docs() {
           results.push(
                RadioProgram::from_document(
                    self.index.searcher().doc(&doc_address).unwrap()
                )
           );
        }

        results
    }
}
