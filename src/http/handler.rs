use super::*;
use HttpMethods::GET;

pub struct WebHandler {}


impl Handler for WebHandler {
    fn handleRequest(&self, request: HttpRequest) -> HttpResponse {
        match request.method {
           GET => {
               match request.path {
                   "/text" => HttpResponse::fromFile("text.txt",ContentType::text),
                   "/json" => HttpResponse::fromFile("text.json",ContentType::json),
                   _ => HttpResponse::new(StatusCode::NotFound),
               }
           }
           _ => self.badRequest(),
        }
    }
}


pub trait Handler {
    fn handleRequest(&self, request : HttpRequest) -> HttpResponse ;
    fn badRequest(&self) -> HttpResponse {
        HttpResponse::new(StatusCode::BadRequest)
    }
}