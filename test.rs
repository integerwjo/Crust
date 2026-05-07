use std::io::Read;
use std::net::TcpStream;

fn main() {
    let mut connection = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut buffer = [0u8; 1024];

    let bytes_read = connection.read(&mut buffer).unwrap();

    println!(
        "Received: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );
}