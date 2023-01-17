// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::prelude::*;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for result in listener.incoming() {
        match result {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buf = [0; 128];
                let n = stream.read(&mut buf).unwrap();
                println!("incoming: {:?}", &buf[..n]);
                let pong = String::from("+PONG\r\n");
                let result = stream.write(pong.as_bytes());
                println!("out: {:?}", result);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
