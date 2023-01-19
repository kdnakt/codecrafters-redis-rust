// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread::spawn;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for result in listener.incoming() {
        spawn(move || {
            match result {
                Ok(mut stream) => {
                    println!("accepted new connection");
                    let mut buf = [0; 128];
                    loop {
                        let n = stream.read(&mut buf).unwrap();
                        let message = std::str::from_utf8(&buf[..n]).unwrap_or("");
                        let req: Vec<&str> = message.split("\r\n").collect();
                        println!("{} {:?}", req[2], req);
                        
                        match req[2] {
                            "command" | "COMMAND" | "ping" | "PONG" => {
                                pong(&mut stream);
                            },
                            "echo" | "ECHO" => {
                                echo(&mut stream, req[4]);
                            },
                            _ => {
                                println!("command not implemented");
                                pong(&mut stream);
                            },
                        }
                        buf = [0; 128];
                    }
                },
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        });
    }
}

fn pong(stream: &mut TcpStream) {
    let pong = String::from("+PONG\r\n");
    let result = stream.write(pong.as_bytes());
    println!("out: {:?}", result);
}

fn echo(stream: &mut TcpStream, message: &str) {
    let len = message.len();
    let msg = format!("${len}\r\n{message}\r\n");
    println!("out: {msg}");
    let result = stream.write(msg.as_bytes());
    println!("out: {:?}", result);
}
