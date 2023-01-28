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
                 let mut conn_open = true;
                 while conn_open{
                     let bytes_read = stream.read(&mut buff).unwrap();
                     if bytes_read == 0{
                        println!("Client has closed the connection!");
                        conn_open = false;
                     }
                     let recieved_string = String::from_utf8(buff.to_vec()).unwrap();
                     println!("String received from other side: {recieved_string}");
                     stream.write("+PONG\r\n".as_bytes()).unwrap();
                 }
             },
             Err(e) => println!("error: {}", e),
         }
    }
}
