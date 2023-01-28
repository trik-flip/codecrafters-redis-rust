use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
         match stream {
             Ok(mut stream) => {
                 println!("accepted new connection");

                 let mut buff = [0;512];
                 stream.read(&mut buff).unwrap();
                 stream.write("+PONG\r\n".as_bytes()).unwrap();
             },
             Err(e) => println!("error: {}", e),
         }
    }
}
