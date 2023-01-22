// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread::spawn;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Cache {
    value: String,
    px: Option<Instant>,
}

impl Cache {
    fn new(value: String) -> Cache {
        Cache {
            value,
            px: None
        }
    }

    fn newpx(value: String, px: i32) -> Cache {
        Cache {
            value,
            px: Some(Instant::now() + Duration::new(0, (px as u32) * 1000000))
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for result in listener.incoming() {
        spawn(move || {
            // TODO: share cache between connections
            let mut cache: HashMap<String, Cache> = HashMap::new();
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
                            "set" | "SET" => {
                                if 10 < req.len() {
                                    // TODO: check if 8th is px
                                    set_px(&mut stream, req[4], req[6], req[10], &mut cache);
                                } else {
                                    set(&mut stream, req[4], req[6], &mut cache);
                                }
                            },
                            "get" | "GET" => {
                                get(&mut stream, req[4], &cache);
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

fn set(stream: &mut TcpStream, key: &str, value: &str, cache: &mut HashMap<String, Cache>) {
    cache.insert(key.to_string(), Cache::new(value.to_string()));
    println!("cache={:?}", cache);
    let msg = format!("$2\r\nOK\r\n");
    println!("out: {msg}");
    let result = stream.write(msg.as_bytes());
    println!("out: {:?}", result);
}

fn set_px(stream: &mut TcpStream, key: &str, value: &str, px: &str, cache: &mut HashMap<String, Cache>) {
    cache.insert(key.to_string(), Cache::newpx(value.to_string(), px.parse::<i32>().unwrap()));
    println!("cache={:?}", cache);
    let msg = format!("$2\r\nOK\r\n");
    println!("out: {msg}");
    let result = stream.write(msg.as_bytes());
    println!("out: {:?}", result);
}

fn get(stream: &mut TcpStream, key: &str, cache: &HashMap<String, Cache>) {
    println!("cache={:?}", cache);
    match cache.get(&key.to_string()) {
        Some(cache) => {
            if let Some(time) = cache.px {
                if Instant::now() < time {
                    let message = &cache.value;
                    let len = message.len();
                    let msg = format!("${len}\r\n{message}\r\n");
                    println!("out: {msg}");
                    let result = stream.write(msg.as_bytes());
                    println!("out: {:?}", result);
                } else {
                    println!("key expired");
                    let msg = "$-1\r\n".to_string();
                    println!("out: {msg}");
                    let result = stream.write(msg.as_bytes());
                    println!("out: {:?}", result);
                }
            } else {
                let message = &cache.value;
                let len = message.len();
                let msg = format!("${len}\r\n{message}\r\n");
                println!("out: {msg}");
                let result = stream.write(msg.as_bytes());
                println!("out: {:?}", result);
            }
        },
        None => {
            println!("key not found");
            let msg = "$-1\r\n".to_string();
            println!("out: {msg}");
            let result = stream.write(msg.as_bytes());
            println!("out: {:?}", result);
        }
    }
}
