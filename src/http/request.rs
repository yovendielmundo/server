use super::method::Method;
use super::method::MethodError;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::str;
use std::str::Utf8Error;

pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<&'buf str>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from<'a>(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        println!("{}, {}, {}", method, path, protocol);

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;
        Ok(Self {
            path: path,
            query_string: Some(path),
            method: method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // let index = request.find(|c: char| c == ' ' || c == '\r')?;
    // Some((&request[..index], &request[index + 1..]));

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidMethod,
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
impl Error for ParseError {}
