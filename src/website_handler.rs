//use super::server::Handler;
use super::http:: {Request, Response, StatusCode, Method, ParseError };
use std::fs;
use std::thread;
use std::time::Duration;


pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {

    pub fn new(public_path: String) -> Self {
        Self {public_path}
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempted: {}" , file_path);
                    None
                }
            }
            Err(_) => None
        }
    }

    pub fn handle_request(&self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")) ,
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                "/sleep" => {
                    thread::sleep(Duration::from_secs(5));
                    Response::new(StatusCode::Ok, self.read_file("hello.html"))
                },
                 // This is a directory traversal vulnerability into the server :D
                path => match self.read_file(path) {
                    Some(i) => Response::new(StatusCode::Ok, Some(i)),
                    None => Response::new(StatusCode::NotFound, None)
                }
            }
            _ => Response::new(StatusCode::BadRequest, None)
        }
    }

    pub fn handle_bad_request(&self, e: &ParseError) -> Response {
        println!("Failed to parse request {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

