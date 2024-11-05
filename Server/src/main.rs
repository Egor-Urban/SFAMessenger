/* 
        SFAM Server
        Version: 0.0.14 a
        Developer: Urban Egor
        License:
*/ 


use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use chrono::prelude::*;


struct Tool;

impl Tool {
    fn get_time() -> String {
        let local_time: DateTime<Local> = Local::now();
        local_time.format("%d/%m/%Y.%H:%M:%S").to_string()
    }


    fn log(xtype: i32, text: String) {
        if xtype == 0 {
            println!("[{}][SERVER][INFO] {}", Tool::get_time(), text);
        }
    }
}



struct Client {
    buffer: [u8; 512],
}

impl Client {
    fn new() -> Self {
        Client {
            buffer: [0; 512],
        }
    }


    fn handle_client(&mut self, mut stream: TcpStream) {
        match stream.read(&mut self.buffer) {
            Ok(bytes_read) => {
                println!("Received a message: {}", String::from_utf8_lossy(&self.buffer[..bytes_read]));

                
                let response = b"PING client";

                if let Err(e) = stream.write_all(response) {
                    println!("!ERROR! Failed to send response to client: {}", e);
                } else {
                    println!("OK Response sent to client.");
                }
            }
            Err(e) => println!("!ERROT! Failed to read from client: {}", e),
        }
    }
}



fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("!ERROR! Can't create server");

    Tool::log(0, "Server started".to_string()); 

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Поток для каждого юзера
                thread::spawn(move || {
                    let mut client = Client::new();
                    client.handle_client(stream);
                });
            }
            Err(e) => {
                println!("!ERROR! Failed to accept a connection: {}", e);
            }
        }
    }
}
