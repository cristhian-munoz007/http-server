use std::convert::TryFrom;
use std::error::Error;
use std::str::Utf8Error;
use std::fmt::{Result as FmtResult, Display, Formatter, Debug};
use std::str;
use super:: {Method, MethodError, QueryString};


// 'a are the usual names of the lifetimes. We'll call it buf to remember this request is tied to the lifetime of the buffer in the server file
// Define basic implementations for traits
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'a> Request<'a> {

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

}

//Names have to match, but not to the top name
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {

    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        /*
        match str::from_utf8(buf) {
            Ok(request) => {},
            Err(_) => return Err(ParseError::InvalidEncoding)
        }
        below is same as above 
        let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;
        and this got even smaller by implementing the From Trait for UTF8 errors to Invalid Encoding errors
        */
        let request = str::from_utf8(buf)?;
        
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;    

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i];    
        //     },
        //     None => {}
        // }
        // Same as above
        // let q = path.find(',');
        // if q.is_some() {
        //     query_string = Some(&path[q.unwrap()+1..]);
        //     path = &path[..q.unwrap()];   
        // }
        //Same as above
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];   
        }
        Ok(Self {
            path,
            query_string,
            method
        })
    }

}



fn get_next_word(request: &str) -> Option<(&str, &str)> {
    
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            // we can assume the +1 since we know ' '  is only 1 byte
            return Some((&request[..i], &request[i+1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method"
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {
}