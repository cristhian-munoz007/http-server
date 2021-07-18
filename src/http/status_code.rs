use std::fmt:: {Display, Formatter, Result as FmtResult};

//Can just derive these 2 because they're basically ints.
 #[derive(Clone, Copy, Debug)]
pub enum StatusCode{
    Ok = 200,
    BadRequest = 400,
    NotFound = 403,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad request",
            Self::NotFound => "Not found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}