use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

use super::Request;
use super::Response;
use super::StatusCode;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Server {
        Server {
            addr
        }
    }

    pub fn run(&self) {
        println!("Listening on {}...", &self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::NotFound,
                                        Some(String::from("<h1>IT WORKS!</h1>")),
                                    )
                                },
                                Err(e) => {
                                    println!("Failed to parse request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                },
                            };
                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        },
                        Err(e) => println!("Failed to read stream: {}", e),
                    }
                },
                Err(e) => println!("Failed to establish connection: {}", e),
            }
        }
    }
}
