use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut connection = TcpStream::connect("127.0.0.1:8080").unwrap();
    println!("Send msg...");

    let mut msg = String::new();
    std::io::stdin().read_line(&mut msg).unwrap();
    println!("Sending: {:?}", msg);
    connection.write(&msg.into_bytes()).unwrap();

    
}