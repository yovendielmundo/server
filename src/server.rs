use crate::http::Request;
use std::{io::Read, net::TcpListener};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        // fn run takes ownership of the struct because
        // of the absence of & before `self` The object will be deallocated when the funtions ends
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
                            match Request::try_from(&buf[..]) {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Failed to parse the request: {}", e)
                                }
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to stablish a connection: {}", e),
            }
        }
    }
}
