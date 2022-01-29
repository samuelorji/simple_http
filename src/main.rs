use std::collections::HashMap;
use std::fmt::{Display, format, Formatter};
use std::io::{Read,Write};
use std::net::TcpListener;
use std::ops::Index;
mod http;
use http::HttpParser;
use http::status_codes::StatusCode;
use http::response::HttpResponse;
use http::{Handler,WebHandler};

fn main() {

    let server = Server {
        address: "127.0.0.1:9000".to_string()
    };

    let handler = WebHandler{};

    server.run(handler);
}


struct Server {
    address : String
}

impl Server {
    fn new(address : String) -> Self {
        Server{
            address
        }
    }

    fn run(&self, mut handler : impl Handler)  {
        loop {
            let listener = TcpListener::bind(&self.address).expect(&format!("cannot bind to {}", &self.address));

           let (mut connection, _ )  = listener.accept().expect("Issue");

            let mut buffer  = [0;1024]; // 1KB buffer


            let read = connection.read(&mut buffer).expect("expect reading");

            let result = std::str::from_utf8(&buffer).expect("cannot parse string");


            let player = &buffer[..read];
           // dbg!("{}",player);

            handler.handleRequest(HttpParser::parseToHttpRequest(player).unwrap()).writeToStream(&mut connection);
            // dbg!(req);
            //
            //
            // let mut headerMap = HashMap::new();
            //
            // headerMap.insert("name","samuel");
            // headerMap.insert("age","16");
            //
            // let resp  = HttpResponse {
            //     statusCode : StatusCode::Ok,
            //     headers :headerMap,
            //     body: Some(String::from(r#"{"name":"samuel"}"#)),
            //     content_type : http::ContentType::json,
            // };
            //
            // //println!("{}",&resp.serialize());
            //
            // //HttpResponse::new(StatusCode::BadRequest).writeToStream(&mut connection);
            //
            // //dbg!(resp);
            //
            // resp.writeToStream(&mut connection);
            // let req2 = HttpParser::parseToHttpRequest2(&result[..read]).unwrap();
            // dbg!(req2);

           // //println!("{}", &result[..read]);
           //  match  parseToHttpRequest(&result[..read]){
           //      Ok(req) =>  println!("http request body as string is {:?}",req),
           //      Err(e) => println!("parsing failed  : {}",e)
           //  }
           //  println!("read {} bytes",read);
        }
    }
}
