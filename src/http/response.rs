use std::collections::HashMap;
use std::net::TcpStream;
use std::io::Write;
use super::*;

#[derive(Debug)]
pub struct HttpResponse<'a >{
    pub statusCode : status_codes::StatusCode,
    pub headers: HashMap<&'a str, &'a str>,
    pub content_type: ContentType,
    pub body: Option<String>

}

impl<'a> HttpResponse<'a> {
    pub fn new(statusCode : status_codes::StatusCode) -> HttpResponse<'a> {
        HttpResponse{
            statusCode,
            headers : HashMap::new(),
            body : None,
            content_type: ContentType::text,
        }
    }

    pub fn writeToStream(self, stream :  &mut TcpStream) {
        stream.write(self.serialize().as_bytes());
    }

    pub fn fromFile(filePath : &str, content_type: ContentType) -> Self {
        match  std::fs::read_to_string(filePath){
            Ok(filestr) => {
               HttpResponse{
                   statusCode: StatusCode::Ok,
                   content_type:content_type,
                   headers: HashMap::new(),
                   body : Some(filestr),
               }
            },
            Err(e) => {
                print!("{}", e);
                HttpResponse::new(StatusCode::BadRequest)
            }
        }

    }

    pub fn serialize(self) -> String {
        let mut header_content = String::new();
        for(k,v) in self.headers.iter() {
            let record = format!("\n{}:{}", k,v);
            header_content.push_str(&record)
        }

        // let emptyarr  =  [0_u8;0];
        // let emptyBody = &emptyarr[..];
        //
        // let content_length = self.body.map(|c| c.len()).get_or_insert(0);


       // let body = self.body.as_ref().map_or(String::new(),|s| *s);
        let body =  self.body.unwrap_or(String::new());

        format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\nContent-Type: {}{}\r\n\r\n{}",
            self.statusCode,
            self.statusCode.as_string(),
            body.len(),
            self.content_type.as_mime_type(),
            header_content,
            body,
        )
    }

}
