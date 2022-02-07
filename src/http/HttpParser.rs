
use super::*;
use std::str;
use std::collections::HashMap;
use std::hash::Hash;


pub fn parseToHttpRequest(bytes: &[u8]) -> Result<HttpRequest, HttpParserErrors> {
    let res = bytes;
    let (method_path_bytes, res) = getMethodStr(res);
    let (header_bytes,res) = getHeaderStr(res);
    let(method_bytes, path_bytes) = getMethodAndPath(method_path_bytes)?;

    // should handle invalid utf-8 case ):
    let method_str = std::str::from_utf8(method_bytes).unwrap();
    let path_str = std::str::from_utf8(path_bytes).unwrap();

    let path_query_vec : Vec<&str>= path_str.split("?").collect();

    let path = path_query_vec[0];
    let query = path_query_vec.get(1).map(|qs| {
        let res=  qs.split("&")
            .collect::<Vec<&str>>().iter()
            .map(|queryStr| {
                let item =  queryStr.split("=").collect::<Vec<&str>>();

                let queryOpt = item.get(1);

                (item[0],*queryOpt.unwrap_or(&""))
            })
            .collect::<HashMap<&str,&str>>();

        res
    }).unwrap_or(HashMap::new());

    let method = match method_str {
        "GET" => Some(HttpMethods::GET),
        "POST" => Some(HttpMethods::POST),
        "PATCH" => Some(HttpMethods::PATCH),
        _ => None,
    }.ok_or(HttpParserErrors::InvalidMethod)?;


    let mut headers: HashMap<&str,&str> = HashMap::new();

    let header_str = std::str::from_utf8(header_bytes).unwrap();

    for line in header_str.split("\n").collect::<Vec<&str>>().iter(){
        let header_vec = line.split(":").collect::<Vec<&str>>();
        headers.insert(header_vec[0].trim(), header_vec[1].trim());
    };


    Ok(HttpRequest {
        method ,
        query,
        body: res,
        headers,
        path
    })
}
fn getMethodStr(bytes : &[u8]) ->  (&[u8], &[u8]){
    let mut count = 0;
    for (index,byte) in bytes.iter().enumerate(){
        if(*byte == 13){
            count = index;
            break;
        }
    }
    (&bytes[..count], &bytes[count + 2 ..])
}

fn getHeaderStr(bytes : &[u8]) ->  (&[u8], &[u8]){
    let mut count = 0;
    for (index,byte) in bytes.iter().enumerate(){
        if(*byte == 13 && bytes[index + 2] == 13_u8){
            // header ends here
            count = index;
            break;
        }
    }
    (&bytes[..count], &bytes[count + 4 ..])
}

fn getMethodAndPath(bytes : &[u8]) ->  Result<(&[u8], &[u8]), HttpParserErrors>{
    let mut count = 0;
    let mut method: Option<&[u8]> = None;
    let mut path: Option<&[u8]> = None;
    for (index, byte) in bytes.iter().enumerate(){
        if (*byte == b' '){
            if(method.is_none()){
                // fix method
                method = Some(&bytes[..index]);
                count = index + 1;
            } else {
                path = Some(&bytes[count .. index]);
                break;
            }
        }

    }

    if(method.is_some() && path.is_some()){
        Ok((method.unwrap(),path.unwrap()))
    } else {
        Err(HttpParserErrors::InvalidRequest)
    }
}

pub fn parseToHttpRequest2(bytes: &str) -> Result<HttpRequest2, HttpParserErrors> {

    let lines = bytes.split("\r\n").collect::<Vec<&str>>();

    let (head, rest) = lines.split_first().ok_or(HttpParserErrors::InvalidRequest)?;


    let mut body_start = 0;
    let mut header_end = 0;
    for(index, elem) in rest.iter().enumerate() {

        if(elem.is_empty()){
            if(header_end == 0){
                // this is header
                header_end = index
            }
        }
    }

    let stuff = head.split(" ").collect::<Vec<&str>>();

    if(stuff.len() != 3){
        return Err(HttpParserErrors::InvalidRequest)
    }

    let method =  match stuff[0] {
        "GET" => Some(HttpMethods::GET),
        "POST" => Some(HttpMethods::POST),
        "PATCH" => Some(HttpMethods::PATCH),
        _ => None,
    }.ok_or(HttpParserErrors::InvalidMethod)?;


    let path_query = stuff[1];
    let other : Vec<&str>= path_query.split("?").collect();

    let path = other[0];
    let query = other.get(1).map(|qs| {
        let res=  qs.split("&")
            .collect::<Vec<&str>>().iter()
            .map(|queryStr| {
                let item =  queryStr.split("=").collect::<Vec<&str>>();
                (item[0],item[1])
            })
            .collect::<HashMap<&str,&str>>();

        res
    }).unwrap_or(HashMap::new());

    //let body = (rest[header_end+1..]);

    // println!("method is {}", stuff[0]);
    // println!("path is {}",stuff[1]);
    // println!("protocol is {}",stuff[2]);
    //println!("headers are {:?}", rest[..header_end].join(""));

    //let bod = rest[header_end+1..]

    let header_string :&str = &rest[..header_end].join("");
    let mut header = HashMap::<&str,&str>::new();

    for line in rest[..header_end].iter() {
        let header_vec = line.split(":").collect::<Vec<&str>>();
        header.insert(header_vec[0].trim(), header_vec[1].trim());
    };

    Ok(HttpRequest2{
        method,
        path ,
        query,
        body: rest[header_end+1..].join(""),
        headers:header,
    }
    )
}



// fn parseToHttpRequest(bytes: &str) -> Result<HttpRequest, HttpParserErrors> {
//
//     let lines = bytes.split("\r\n").collect::<Vec<&str>>();
//
//     let (head, rest) = lines.split_first().ok_or(HttpParserErrors::InvalidRequest)?;
//
//     let mut body_start = 0;
//     let mut header_end = 0;
//     for(index, elem) in rest.iter().enumerate() {
//
//         if(elem.is_empty()){
//             if(header_end == 0){
//                 // this is header
//                 header_end = index
//             }
//         }
//     }
//
//     let stuff = head.split(" ").collect::<Vec<&str>>();
//
//     if(stuff.len() != 3){
//         return Err(HttpParserErrors::InvalidRequest)
//     }
//
//    let method =  match stuff[0] {
//          "GET" => Some(HttpMethods::GET),
//          "POST" => Some(HttpMethods::POST),
//          "PATCH" => Some(HttpMethods::PATCH),
//          _ => None,
//     }.ok_or(HttpParserErrors::InvalidMethod)?;
//
//
//     let path_query = stuff[1];
//     let other : Vec<&str>= path_query.split("?").collect();
//
//     let path = other[0];
//     let query = other.get(1).map(|qs| {
//             let res=  qs.split("&")
//             .collect::<Vec<&str>>().iter()
//                 .map(|queryStr| {
//                     let item =  queryStr.split("=").collect::<Vec<&str>>();
//                     (item[0],item[1])
//                 })
//            .collect::<HashMap<&str,&str>>();
//
//         res
//     }).unwrap_or(HashMap::new());
//
//     //let body = (rest[header_end+1..]);
//
//     // println!("method is {}", stuff[0]);
//     // println!("path is {}",stuff[1]);
//     // println!("protocol is {}",stuff[2]);
//     //println!("headers are {:?}", rest[..header_end].join(""));
//
//     //let bod = rest[header_end+1..]
//
//     let header_string :&str = &rest[..header_end].join("");
//     let mut header = HashMap::<&str,&str>::new();
//
//     for line in rest[..header_end].iter() {
//         let header_vec = line.split(":").collect::<Vec<&str>>();
//         header.insert(header_vec[0].trim(), header_vec[1].trim());
//     };
//
//     Ok(HttpRequest{
//         method,
//         path ,
//         query,
//         body: (rest[header_end+1..]).join(""),
//         headers:header,
//     }
//     )
// }