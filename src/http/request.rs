use super::*;
use std::collections::HashMap;
#[derive(Debug)]
pub struct HttpRequest<'a> {
    pub method : HttpMethods,
    pub path : &'a str,
    pub query: HashMap<&'a str, &'a str>,
    pub headers: HashMap<&'a str, &'a str>,
    pub body: &'a [u8]
}

#[derive(Debug)]
pub struct HttpRequest2<'a> {
    pub method : HttpMethods,
    pub path : &'a str,
    pub query: HashMap<&'a str, &'a str>,
    pub headers: HashMap<&'a str, &'a str>,
    pub body: String
}

