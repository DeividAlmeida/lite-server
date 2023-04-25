extern crate tiny_http;

use std::sync::Arc;
use std::thread;

fn main() {
    let server = Arc::new(tiny_http::Server::http("0.0.0.0:8080").unwrap());
    let server = server.clone();

    let thread=  thread::spawn(move || {
      for request in server.incoming_requests() {
        match request.url() {
          "/" => {
              let response = tiny_http::Response::from_string("Hello, world!".to_string());
              let _ = request.respond(response);
          }
          "/about" => {
              let response = tiny_http::Response::from_string("About page".to_string());
              let _ = request.respond(response);
          }
          _ => {
              let response = tiny_http::Response::from_string("404 Not Found".to_string())
                  .with_status_code(tiny_http::StatusCode::from(404));
              let _ = request.respond(response);
          }
        }
      }
    });
  
    thread.join().unwrap();

}