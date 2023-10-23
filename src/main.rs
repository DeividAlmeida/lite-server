use std::{sync::Arc, thread};
use serde_json;
use tiny_http::{ Response, Server, StatusCode, Method, Header, };
use tokio;

mod services;
mod structs;

use crate::structs::{publisher::Publisher, request_data::RequestData};

#[tokio::main]
async fn main() {
  let server = Arc::new(Server::http("0.0.0.0:80").unwrap());
  let server = server.clone();

  let thread=   thread::spawn(move || {
    for mut request in server.incoming_requests() {
      match (request.method(), request.url()) {
        (&Method::Get, "/") => {
            let response = Response::from_string("Hello world!".to_string());
            let _ = request.respond(response);
        }

        //pega todos os publishers
        (&Method::Get, "/publisher") => {
          let response = services::list_publisher();
          match response {
            Ok(value) =>{
              let response = Response::from_string(value)
              .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
              let _ = request.respond(response);
            } 
            Err(erro) =>{
              let response = Response::from_string(erro.to_string()).with_status_code(StatusCode::from(404));
              let _ = request.respond(response);
            } 
          }
        }

        //pega um publisher especifico
        (&Method::Get, path) if path.starts_with("/publisher/") => {

          let id = request.url().trim_start_matches("/publisher/");
          let response = services::get_publisher(id);

          match response {
            Ok(value) =>{
              let response = Response::from_string(value)
              .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
              let _ = request.respond(response);
            }
            Err(erro) =>{
              let response = Response::from_string(erro.to_string()).with_status_code(StatusCode::from(404));
              let _ = request.respond(response);
            } 
          }
        }

        // cria um novo publisher
        (&Method::Post, "/publisher") => {
          let publisher: Publisher = serde_json::from_reader(request.as_reader()).unwrap();

          let response = services::create_publisher(publisher);
          match response {
            Ok(value) =>{
              let response = Response::from_string(value.to_string());
              let _ = request.respond(response);
            } 
            Err(erro) =>{
              let response = Response::from_string(erro.to_string()).with_status_code(StatusCode::from(404));
              let _ = request.respond(response);
            } 
          }
        }

        // deleta um publisher
        (&Method::Delete, path) if path.starts_with("/publisher/") => {

          let id = request.url().trim_start_matches("/publisher/");
          let response = services::delete_publisher(id);

          match response {

            Ok(value) =>{
              let response = Response::from_string(value.to_string())
              .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
              
              let _ = request.respond(response);
            }

            Err(erro) =>{
              let response = Response::from_string(erro.to_string()).with_status_code(StatusCode::from(404));
              let _ = request.respond(response);
            } 
          }
        }

        //atualizar um publisher especifico
        (&Method::Patch, path) if path.starts_with("/publisher/") => {

          let publisher: Publisher = serde_json::from_reader(request.as_reader()).unwrap();
          let id = request.url().trim_start_matches("/publisher/");
          let response = services::update_publisher(id, publisher);
        
          match response {
            Ok(value) =>{
              let response = Response::from_string(value.to_string())
              .with_header(Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap());
              let _ = request.respond(response);
            }
            Err(erro) =>{
              let response = Response::from_string(erro.to_string()).with_status_code(StatusCode::from(404));
              let _ = request.respond(response);
            } 
          }

        }
        
        // cria novas apresentações
        (&Method::Post, path) if path.starts_with("/presentations") => {
          let data: RequestData = match serde_json::from_reader(request.as_reader()){
            Ok(value) => value,
            Err(erro) => {
              let response = Response::from_string(erro.to_string()).with_status_code(StatusCode::from(404));
              let _ = request.respond(response);
              continue;
            }
          };
          
          let length = data.length;
          let gender = data.gender.to_string();
          
          match services::create_presentations(length, gender) {
            Ok(value) => {
              let response = Response::from_string(value.to_string());
              let _ = request.respond(response);
            } 
            Err(erro) =>{
              let response = Response::from_string(erro.to_string()).with_status_code(StatusCode::from(404));
              let _ = request.respond(response);
            } 
          }
        }

        _ => {
            let response = Response::from_string("404 Not Found".to_string())
                .with_status_code(StatusCode::from(404));
            let _ = request.respond(response);
        }
      }
    }
  });

  thread.join().unwrap();

}
