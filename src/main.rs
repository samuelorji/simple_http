use std::collections::HashMap;
use std::fmt::{Display, format, Formatter};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::ops::Index;


mod http;
mod threading;

use http::HttpParser;
use http::status_codes::StatusCode;
use http::response::HttpResponse;
use http::{Handler, WebHandler};
use threading::threadpool;
use crate::threadpool::ThreadPool;


fn main() {
    let server = Server {
        address: "127.0.0.1:9000".to_string()
    };


    let pool = threadpool::ThreadPool::new(4u8);
    let handler = WebHandler {};

    server.run(handler, pool);
}


struct Server {
    address: String,
}

impl Server {
    fn new(address: String) -> Self {
        Server {
            address
        }
    }
    fn handleConnection(mut stream: &mut std::net::TcpStream) -> () {
        let mut buffer = [0; 1024]; // 1KB buffer
        let handler = WebHandler{};


        let read = stream.read(&mut buffer).expect("expect reading");


        let player = &buffer[..read];
        // dbg!("{}",player);

        handler.handleRequest(HttpParser::parseToHttpRequest(player).unwrap()).writeToStream(&mut stream);
    }



    fn run(&self, handler: impl Handler, pool: ThreadPool) {
        loop {
            let listener = TcpListener::bind(&self.address).expect(&format!("cannot bind to {}", &self.address));

            let (mut connection, _) = listener.accept().expect("Issue");

            pool.execute(move ||Server::handleConnection(& mut connection));

       }
    }
}
