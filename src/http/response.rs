use super::StatusCode;
use std::io:: {Write, Result as IoResult};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response {status_code, body}
    }

    //pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
    //dyn stands for dynamic dispatch. Will resolve at runtime the correct impl for Write
    //uss V-Tables to figure which function to use. Takes a bit time becuase of the jumping from addresses
    //    pub fn send(&self, stream: &mut dyn Write) -> IoResult<()>  {
    // impl will use a static dispatch. Aka will figure out the right implementation at compile time.
    // for every call to this function. The compiler will copy the code for each specific case.
    // so somethign like this will get generated: pub fn send_TcpStream(&self, stream: &mut TcpStream) -> IoResult<()>  { 
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()>  { 
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        stream.flush().unwrap();
        write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", 
            self.status_code, 
            self.status_code.reason_phrase(),
            body
        )
    }
}

// This is all replaced by send, so that we don't have to write the body
// to the heap. Instead we'll write directly to the tcp stream
//impl Display for Response {
//     fn fmt(&self, f: &mut Formatter) -> FmtResult {
//         let body = match &self.body {
//             Some(b) => b,
//             None => ""
//         };
//         write!(f, "HTTP/1.1 {} {}\r\n\r\n{}", 
//             self.status_code, 
//             self.status_code.reason_phrase(),
//             body
//         )
//     }
// }
