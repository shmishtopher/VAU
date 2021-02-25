//! Copyright 2021 Christopher K. Schmitt "Shmish"
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//! 
//!     http://www.apache.org/licenses/LICENSE-2.0
//! 
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.


use rust_embed::RustEmbed;
use actix_web::rt::System;
use actix_web::web;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use std::sync::mpsc;
use std::thread;
use web_view::Content;
use mime_guess::from_path;


#[derive(RustEmbed)]
#[folder = "gui/assets"]
struct Asset;


fn file(path: &str) -> HttpResponse {
    if let Some(content) = Asset::get(path) {
        let body = content.into_owned();
        HttpResponse::Ok()
            .content_type(from_path(path).first_or_octet_stream().as_ref())
            .body(body)
    }
    else {
        HttpResponse::NotFound().body("404 - Not Found")
    }
}


#[actix_web::main]
async fn main() {
    // Cross-thread channels
    let (server_tx, server_rx) = mpsc::channel();
    let (address_tx, address_rx) = mpsc::channel();

    // Spin up webserver in a seperate thread
    thread::spawn(move || {
        let system = System::new("http_server");
        let server = HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(|| file("index.html")))
                .route("/styles/reset.css", web::get().to(|| file("styles/reset.css")))
                .route("/styles/layout.css", web::get().to(|| file("styles/layout.css")))
                .route("/styles/components.css", web::get().to(|| file("styles/components.css")))
                .route("/styles/palette.css", web::get().to(|| file("styles/palette.css")))
                .route("/scripts/ui.js", web::get().to(|| file("scripts/ui.js")))
                .route("/scripts/rpc.js", web::get().to(|| file("scripts/rpc.js")))
                .route("/media/topography.svg", web::get().to(|| file("media/topography.svg")))
        })
        .bind("127.0.0.1:0")
        .unwrap();

        let address = server.addrs().first().unwrap().to_string();
        let server = server.run();
        
        server_tx.send(server).unwrap();
        address_tx.send(address).unwrap();

        system.run().unwrap();
    });

    // Pull port and server from thread
    let server = server_rx.recv().unwrap();
    let address = address_rx.recv().unwrap();

    println!("{}", address);

    // Setup webview
    web_view::builder()
        .debug(true)
        .title("VAU")
        .size(1200, 600)
        .resizable(false)
        .content(Content::Url(format!("http://{}", address)))
        .user_data(())
        .invoke_handler(|_, _| Ok(()))
        .run()
        .unwrap();

    // Shut down the webserver
    server.stop(true).await;
}
