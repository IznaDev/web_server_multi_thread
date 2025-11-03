use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

use web_server_multi_threaded::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

// Function to handle incoming TCP connections
pub fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 512];
    stream.read(&mut buffer).unwrap(); // move the data from the stream into the buffer
    println!("Request: {}", String::from_utf8_lossy(&buffer[..])); // each non-utf8 character is replaced with � thanks to from_utf8_lossy

    let get = b"GET / HTTP/1.1\r\n";
    //compare the request in the buffer to the expected GET request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
