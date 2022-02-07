use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let method = match s {
            "GET" => Self::GET,
            "DELETE" => Self::DELETE,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "HEAD" => Self::HEAD,
            "CONNECT" => Self::CONNECT,
            "OPTIONS" => Self::OPTIONS,
            "TRACE" => Self::TRACE,
            "PATCH" => Self::PATCH,
            _ => return Err(MethodError),
        };
        Ok(method)
    }
}

pub struct MethodError;