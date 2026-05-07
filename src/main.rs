use std::{
        fs::read, io::{ self, BufRead, Write},
        net::{TcpListener, TcpStream}, 
        str::Bytes, 
        sync::{Arc, Mutex, mpsc:: {self, Receiver, Sender, channel}},
        thread
    };

fn main() {

    let listener = TcpListener::bind("127.0.0.1:8080");
    let mut connections: Vec<TcpStream> = vec![];
    let (sender, receiver) = mpsc::channel();
    //let (sender, receiver) = channel()::<u8>;
    let receiver  = Arc::new(Mutex::new(receiver));


    match listener {
        Ok(listener) => {
           for connection in listener.incoming() {
               let mut connection = connection.unwrap();

               // cloned for sending messages back to this client
               let this_receiver = Arc::clone(&receiver);
               connections.push(connection.try_clone().expect("Failed to clone stream"));
               let this_sender = sender.clone();
                thread::spawn( move || {
                    handle_connection(&mut connection, this_sender, this_receiver);
                });
           }
        }

        Err(e) => {
            println!("Error occured: {}", e)
        }
    }



    // The main thread receives messages from all spawned threads and broadcasts them
    for message in receiver.lock().unwrap().recv() {
        for connection in &mut connections {
            connection.write_all(&message).unwrap()
        }
    }
    
}


fn handle_connection(
    stream: &mut TcpStream,
    sender: Sender<Vec<u8>>,
    receiver: Arc<Mutex<Receiver<Vec<u8>>>>,
) {
    println!("Got stream: {:?}", stream);
    println!("Type message...");

    let mut input = String::new();

    // Read input from stdin
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Send through channel
    sender
        .send(input.into_bytes())
        .expect("Failed to send message");

    loop {
        // Receive message from channel
        let msg = receiver
            .lock()
            .unwrap()
            .recv()
            .expect("Failed to receive message");

        // Write message to TCP stream
        stream
            .write_all(&msg)
            .expect("Failed to write to stream");
    }
}