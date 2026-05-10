use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let mut buffer = [0; 1024];
    let mut cloned_stream = stream.try_clone().expect("Failed to clone steam");
    let handle = std::thread::spawn( move || {
        let mut buffer = [0; 1024];
        let bytes_read = cloned_stream.read(&mut buffer).expect("Failed to read from stream");
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Sent in chat: {}",message);
    });
    
    let bytes_read = stream.read(&mut buffer).expect("Failed to read");
    let msg = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Read: {:?}", msg);

    println!("Type a message to send...");
    let mut message = String::new();
    std::io::stdin().read_line(&mut message).expect("Failed to read input");
    println!("sending: {}", message);
    stream.write_all(message.as_bytes()).expect("Failed to write to stream");



    handle.join().unwrap();
    
}