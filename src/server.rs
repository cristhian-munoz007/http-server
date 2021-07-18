use std::net::{TcpStream, TcpListener};
use crate::http:: {Request};//, Response, StatusCode, ParseError};
use std::io::Read;
use std::convert::TryFrom;
use super::thread_pool::ThreadPool;
use super::website_handler::WebsiteHandler;
//crate -> root path


// pub trait Handler {
//     fn handle_request(&self, request: &Request) -> Response;

//     fn handle_bad_request(&self, e: &ParseError) -> Response {
//         println!("Failed to parse request {}", e);
//         Response::new(StatusCode::BadRequest, None)
//     }
// }

pub struct Server {
    addr: String
}

impl Server {
    // This is an associated function
    pub fn new(addr: String) -> Server {
        Server { addr }
    }

    /* This is a method, so takes the self param
    Remember: run(self) would be taking ownership of the struc, and then deallocating it when the 
    execution of run finishes. If we don't want that, then we should pass the reference instead run(&self) */
    pub fn run(self, public_path: String) {
        println!("Server is listennig on: {}", self.addr);   
        let listener: TcpListener = TcpListener::bind(&self.addr).unwrap();
        let pool = ThreadPool::new(5);
        
        loop {
            match listener.accept() {
                Ok((tcp_stream, _)) => {
                    let public_path = public_path.clone();
                    pool.execute(|| {
                        handle_connection(tcp_stream, public_path);
                    })           
                },
                Err(e) => println!("Failed to establish a connection: {}", e)
            } 
        }
    }

}

fn handle_connection(mut tcp_stream: TcpStream, public_path: String) {
    println!("Established a connection");
    let mut buffer = [0; 1024];
    let handler = WebsiteHandler::new(public_path.to_string());
    // what if the socket has more than 1024. This would blow up, so this ain't prod reay
    match tcp_stream.read(&mut buffer) {
        Ok(_) => {
            //println!("Received a request: {}", String::from_utf8_lossy(&buffer));

            //Convert into a generic slice. Otherwise it complains about the size;
            let response = match Request::try_from(&buffer[..]) {
                Ok(request) => {
                    handler.handle_request(&request)
                }
                Err(e) => {
                   handler.handle_bad_request(&e)
                }
            };

            if let Err(e) = response.send(&mut tcp_stream) {
                println!("Error sending response {}", e);
            }

            println!("Done")

        },
        Err(e) => print!("Failed ro read from connection: {}", e)
    }
}
