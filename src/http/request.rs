use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::str;

use super::Method;
use super::MethodError;
use super::QueryString;

/**
 * GET /path?query=param HTTP/1.1 \r\n
 * HEADERS \r\n
 * BODY \r\n
 */
#[derive(Debug)]
pub struct Request<'buf> {
    method: Method,
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    // headers: String, // TODO: hashmap
    // body: String,
}

// 'buf lifetime parameter is necessary so that
// HTTP request buffer is not deallocated from heap
// while the pointer references exist with the same
// lifetime parameter
// 😡
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buf)?;
        let (method, request) = get_next_word(&request).ok_or(Self::Error::InvalidRequest)?;
        let (mut path, request) = get_next_word(&request).ok_or(Self::Error::InvalidRequest)?;
        let (protocol, _) = get_next_word(&request).ok_or(Self::Error::InvalidRequest)?;
        if protocol != "HTTP/1.1" {
            return Err(Self::Error::InvalidProtocol);
        }
        let method: Method = method.parse()?;
        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
            // FIXME: thread 'main' panicked at 'byte index
            // 2 is out of bounds of `/`', src/http/request.rs:44:52
        };
        Ok(Self {
            method,
            path,
            query_string,
        })
    }
    
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        match c {
            ' ' | '\r' => return Some((&request[..i], &request[i+1..])),
            _ => {},
        }
    }
    None
}

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
impl Error for ParseError {}


// Convert Utf8Error to ParseError when using ? operator
impl From<str::Utf8Error> for ParseError {
    fn from(_: str::Utf8Error) -> Self {
        return ParseError::InvalidEncoding;
    }
}

// Convert MethodError to ParseError when using ? operator
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        return ParseError::InvalidMethod;
    }
}