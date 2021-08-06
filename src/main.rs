use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use rust_server::ThreadPool;

fn main() {
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream))
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "src/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };

    let content = fs::read_to_string(filename).unwrap();
    let res = format!(
        "{}\r\nContent-Length:{}\r\n\r\n{}",
        status,
        content.len(),
        content
    );
    stream.write(res.as_bytes()).unwrap();
    stream.flush().unwrap();
}
