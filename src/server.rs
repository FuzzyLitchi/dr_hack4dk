use hyper;
use futures;

use futures::future::Future;
use hyper::{Method, StatusCode};
use hyper::server::{Http, Request, Response, Service};
use searching::Searcher;
use serde_json;

pub struct Server {
    searcher: Searcher,
}

impl Server {
    fn default() -> Server {
        Server {
            searcher: Searcher::default(),
        }
    }

    pub fn serve() {
        let addr = "127.0.0.1:3000".parse().unwrap();
        let server = Http::new().bind(&addr, || Ok(Server::default())).unwrap();
        server.run().unwrap();
    }
}

impl Service for Server {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

         match (req.method(), req.path()) {
            (&Method::Get, "/query") => {
                if let Some(query_string) = req.query() {
                    response.set_body(
                        serde_json::to_string(
                            &self.searcher.search(&query_string, 1)[0]
                        ).unwrap()
                );
                } else {
                    response.set_body("You have to query it")
                }
            },
            (&Method::Post, "/echo") => {
                // we'll be back
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };

        Box::new(futures::future::ok(response))
    }
}
