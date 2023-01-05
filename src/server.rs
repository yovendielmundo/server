use std::{io::Read, io::Write, net::TcpListener};

use crate::http::{Request, Response, StatusCode};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        // fn run takes ownership of the struct because
        // of the absence of & before `self` The object will be deallocated when the functions ends
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    let mut buf = [0; 1024];
                    match stream.read(&mut buf) {
                        Ok(nbytes) => {
                            println!(
                                "Received a request. addr:[{}] nbytes:[{}]",
                                addr.to_string(),
                                nbytes
                            );
                            println!("{}", String::from_utf8_lossy(&buf));

                            let response = match Request::try_from(&buf[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>It Works!!</h1>".to_string()),
                                    )
                                }
                                Err(e) => {
                                    println!("Failed to parse the request: {}", e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e)
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
