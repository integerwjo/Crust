use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{
        Arc, Mutex,
        mpsc::{self, Sender},
    },
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("Failed to bind");

    let (tx, rx) = mpsc::channel::<String>();

    // shared list of clients
    let clients: Arc<Mutex<Vec<TcpStream>>> =
        Arc::new(Mutex::new(Vec::new()));

    let clients_broadcast = Arc::clone(&clients);

    // BROADCAST THREAD
    thread::spawn(move || {
        while let Ok(msg) = rx.recv() {
            println!("Broadcasting: {}", msg);

            let mut clients = clients_broadcast.lock().unwrap();

            clients.retain(|client| {
                let mut client = client.try_clone().unwrap();

                match client.write_all(msg.as_bytes()) {
                    Ok(_) => true,
                    Err(_) => false, // drop dead connections
                }
            });
        }
    });

    // ACCEPT LOOP
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let tx = tx.clone();
        let clients = Arc::clone(&clients);

        // store client
        clients.lock().unwrap().push(stream.try_clone().unwrap());

        thread::spawn(move || {
            handle_connection(stream, tx);
        });
    }
}

fn handle_connection(stream: TcpStream, sender: Sender<String>) {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();

        let bytes = reader.read_line(&mut line).unwrap();
        if bytes == 0 {
            break;
        }

        sender.send(line.clone()).unwrap();
    }
}