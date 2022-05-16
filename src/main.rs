#![allow(unused)]
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
mod lib;
use std::fs;
fn main() {
    // bind tcp to 3000 on LH
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    // allow 4 threads max in the pool
    let pool = lib::ThreadPool::new(4);
    // stream requests from tcp into handler when sent
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // handle tcp stream requests by reading the buffer in as byte array (hence 8 bit unsigned)
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // this is for a byte string literal the b".." thing
    let get = b"GET / HTTP/1.1\r\n";

    // if get request from http
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").unwrap();

        // write a response in the form of a string denotating an HTTP type of text incoming with 200 status from server on the clientside
        // then put in the fixed size content
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        // write it as a byte array to the stream
        stream.write(response.as_bytes()).unwrap();
        // close connection
        stream.flush().unwrap();
    } else {
        // write 404
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}