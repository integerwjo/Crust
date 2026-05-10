use std::{
          net::{TcpStream, TcpListener},
          sync::{mpsc::{self, Sender, Receiver}, {Arc, Mutex}},
          thread,
          io::{Write,Read},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to port");

    let (tx, _rx) = mpsc::channel::<String>();
    // holds all the streams
    let clients: Arc::<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();



    // This thread will listen for incomming messages and print them
    // 
    /*
    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            for client in clients {
                client.write_all(msg.as_bytes()).expect("Failed to broadcast");
            }
        }
        
    }); */
    
    for stream in listener.incoming() {
        // clone the transmitter
        let streams = Arc::clone(&clients);
        let transmitter = tx.clone();
        let stream = stream.unwrap();
        let stream_clone = stream.try_clone().expect("Cloning stream failed");
        clients.lock().unwrap().push(stream_clone);

        // a thread to handle incoming connection
        let handle = thread::spawn( move || {
            handle_stream(stream, transmitter, streams);
        });
        handles.push(handle);
    }


    for handle in handles {
        handle.join().unwrap();
    }
}

fn handle_stream(mut stream: TcpStream, _transmitter: Sender<String>,
    clients: Arc<std::sync::Mutex<Vec<TcpStream>>>) {
    println!("{:?}", stream);
    println!("Our side of the connection: {:?}", stream.local_addr());
    println!("The other side of the connection: {:?}", stream.peer_addr());

    let message = String::from("Server says: Connection successful");

    // converting message to bytes
    let bytes = message.as_bytes();
    // writing a byte literal to the stream
    // stream.write_all(b"Hello world").expect("Failed to write to stream");
    // 
    stream.write_all(bytes).unwrap();

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
    let received = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Server received: {}", received);

    // This thread will now send the message it has received to the main thread
    // for broadcasting
    // transmitter.send(received.to_string()).expect("Failed to transmit message");
    // 
    // instead of sending it to the main thread, we let the spawned threads handle 
    // writing because they have access to the stream

    let clients = clients.lock().unwrap();

    for mut client in clients.iter() {
        client.write_all(received.as_bytes()).expect("Failed to transmit");
    }
    
   
}