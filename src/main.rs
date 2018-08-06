extern crate http_server;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::thread;
use std::time::Duration;

use http_server::ThreadPool;

fn main (){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);


    for stream in listener.incoming() {
        let stream = stream.unwrap(); 

        pool.execute(|| {
            handle_connection(stream);
        });
     }
}

enum ContentType{
    TEXT,
    BINARY,
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512]; // 512 bytes
    stream.read(&mut buffer).unwrap(); 

    println!("Request: {}", String::from_utf8_lossy(&buffer));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let json = b"GET /json HTTP/1.1\r\n";
    let image = b"GET /image.png HTTP/1.1\r\n";

    let (status_line, filename, content_type) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html",ContentType::TEXT)
    } else if buffer.starts_with(sleep) { 
        thread::sleep(Duration::from_secs(5)); 
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html", ContentType::TEXT)
    } else if buffer.starts_with(json){
        ("HTTP/1.1 200 OK\r\nContent-Type:application/json;charset=UTF-8\r\n\r\n", "hello.json", ContentType::TEXT)
    } else if buffer.starts_with(image) {
        ("HTTP/1.1 200 OK\r\nContent-Length:853\r\nContent-Type:image/png\r\n\r\n","image.png", ContentType::BINARY)
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html", ContentType::TEXT)
    };

    
    match content_type {
        ContentType::TEXT => {
            let contents = fs::read_to_string(filename).unwrap();
            let response = format!("{}{}", status_line, contents);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
        ContentType::BINARY =>{
            let mut header = status_line.as_bytes().to_vec();
            let mut contents = fs::read(filename).unwrap();
            header.append(&mut contents);
            stream.write(header.as_slice()).unwrap();
            stream.flush().unwrap();
        }
    }
}



