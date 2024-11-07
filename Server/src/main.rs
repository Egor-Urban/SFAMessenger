/* 
        SFAM Server
        Version: 0.0.236 a
        Developer: Urban Egor
*/ 

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fmt;

use chrono::prelude::*;



#[derive(Debug)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Debug => write!(f, "DEBUG"),
        }
    }
}

fn log(level: LogLevel, msg: &str, args: &[&str]) {
    let now = Tool::get_time();

    // Подставляем аргументы вручную
    let mut formatted_msg = msg.to_string();
    for arg in args {
        if let Some(pos) = formatted_msg.find("{}") {
            formatted_msg.replace_range(pos..pos + 2, arg);
        }
    }

    println!(
        "[{}][SERVER][{}] {}",
        now,
        level,
        formatted_msg
    );
}



struct Tool;
impl Tool {
    fn get_time() -> String {
        let local_time: DateTime<Local> = Local::now();
        local_time.format("%d/%m/%Y.%H:%M:%S").to_string()
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


    fn get_client_ip(stream: &TcpStream) -> String {
        match stream.peer_addr() {
            Ok(addr) => addr.to_string(),      
            Err(_) => "Unknown IP".to_string(),
        }
    }


    fn handle_client(&mut self, mut stream: TcpStream) {
        let client_ip = Self::get_client_ip(&stream);

        //println!("{} {} connected to server.", Tool::log(0), client_ip);


        match stream.read(&mut self.buffer) {
            Ok(bytes_read) => {
                let received_message = String::from_utf8_lossy(&self.buffer[..bytes_read]);
                //println!("Received a message {}: {}", client_ip, received_message);
                log(LogLevel::Info, "Received a message from {}: {}", &[&client_ip, &received_message]);
                

                let response = b"PING client";
                let response_str = match String::from_utf8(response.to_vec()) {
                    Ok(s) => s,
                    Err(e) => {
                        log(LogLevel::Error, "Translating var error: {}", &[&format!("{}", e)]);
                        return;
                    }
                };

                if let Err(e) = stream.write_all(response) {
                    log(LogLevel::Error, "Failed to send response to client: {}", &[&format!("{}", e)]);
                } else {
                    log(LogLevel::Info, "Response sent to {}: {}", &[&client_ip, &response_str]);
                }
            }
            Err(e) => println!("!ERROR! Failed to read from client: {}", e),
        }
    }
}



fn main() {
    let server_port: String = "7878".to_owned();
    let server_ip: String = format!("127.0.0.1:{}", server_port).to_owned();
    let listener = TcpListener::bind(server_ip.to_string()).expect("!ERROR! Can't create server");
    
    
    //log(LogLevel::Info, "{} connected to server.", &["127.0.0.1:50115"]);
    log(LogLevel::Info, "Server started on port {}", &[&server_port]);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log(LogLevel::Info, "Connecting...", &[]);
                // Поток для каждого юзера
                thread::spawn(move || {
                    let mut client = Client::new();
                    client.handle_client(stream);
                });
            }
            Err(e) => {
                log(LogLevel::Info, "Failed to accept connection:", &[]);
                println!("{}", e);
            }
        }
    }
}
