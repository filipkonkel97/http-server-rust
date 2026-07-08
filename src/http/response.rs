use super::StatusCode;
use std::fmt::Formatter;
use std::fmt::{Display, Result as FmtResult};
use std::io::{Result as IoResutl, Write};
use std::net::TcpStream;
use std::net::ToSocketAddrs;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
    pub fn send(&self, stream: &mut impl Write) -> IoResutl<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {} \r\n\r\n",
            self.status_code,
            self.status_code.reason_phrase(),
        )
    }
}
