use std::fmt::{Display, format, Formatter};
#[derive(Debug)]
pub enum HttpParserErrors {
    InvalidProtocol,
    InvalidRequest,
    InvalidMethod,

}

impl Display for HttpParserErrors {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let err =  { match self {
            HttpParserErrors::InvalidProtocol => "Invalid Protocol",
            HttpParserErrors::InvalidRequest => "Invalid Request",
            HttpParserErrors::InvalidMethod => "Invalid Method",
            _ => "Nothing",
        }};
        writeln!(formatter, "{}",err)

    }
}