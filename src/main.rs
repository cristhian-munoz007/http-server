#![allow(dead_code)]

use server::Server;
use std::env;

mod http;
mod server;
mod website_handler;
mod thread_pool;


fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("{}", public_path);
    let server = Server::new("127.0.0.1:8080".to_string());
    // I actually don't want the handler to be shared. It blocks
    server.run(public_path);
}

/* 
Possible enhancements:
1 - Extend functionality of Request/Response to handle HTTP headers
2 - Make the server be multithreaded using std::thread and std::sync 
3 - a Make it asynchronous :O!!! (tokyo.rs)
*/