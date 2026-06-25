use std::str::FromStr;
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
        match s {
            "GET" => Ok(self::GET),
            "DELETE" => Ok(self::DELETE),
            "POST" => Ok(self::POST),
            "PUT" => Ok(self::PUT),
            "HEAD" => Ok(self::HEAD),
            "CONNECT" => Ok(self::CONNECT),
            "OPTIONS" => Ok(self::OPTIONS),
            "TRACE" => Ok(self::TRACE),
            "PATCH" => Ok(self::PATCH),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;
