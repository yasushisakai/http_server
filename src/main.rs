extern crate http_server;
extern crate regex;
extern crate image;
extern crate chrono;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::sync::{Arc, Mutex};
use regex::Regex;

use http_server::ThreadPool;

mod state;
mod helper;
mod color_table;

use state::State; 

fn main (){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(10);

    let mut state = Arc::new(Mutex::new(State::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap(); 
        let state = Arc::clone(&state);

        pool.execute(move || {
            handle_connection(stream, state);
        });
     }
}

fn handle_connection(mut stream: TcpStream, state: Arc<Mutex<State>>){
    let mut buffer = [0; 512]; // 512 bytes
    stream.read(&mut buffer).unwrap(); 

    let home = b"GET /talkingdrums/ HTTP/1.1\r\n";
    let status_json = b"GET /talkingdrums/status/ HTTP/1.1\r\n";
    let send_pixels = b"GET /talkingdrums/image/send/";
    let image_original = b"GET /talkingdrums/image/original/ HTTP/1.1\r\n";
    let image_current = b"GET /talkingdrums/image/current/ HTTP/1.1\r\n";
    let get_pixels = b"GET /talkingdrums/image/get/next/ HTTP/1.1\r\n";
    let start = b"GET /talkingdrums/image/start/ HTTP/1.1\r\n";
    let stop = b"GET /talkingdrums/image/stop/ HTTP/1.1\r\n";
    let reset = b"GET /talkingdrums/image/reset/ HTTP/1.1\r\n";
  
    let mut response: Vec<u8> = Vec::new();
    
    if buffer.starts_with(home){
        response = helper::make_ok_header();
        response.extend(b"\r\n");
        let contents = fs::read("hello.html").unwrap();
        response.extend(contents);
    } else if buffer.starts_with(status_json) {
        response = helper::make_json_header();
        let state_locked = state.lock().unwrap();
        let contents = serde_json::to_vec(&*state_locked).unwrap();
        response.extend(contents);
    } else if buffer.starts_with(send_pixels) {
        // convert buffer to string    
        let buffer = String::from_utf8(buffer.to_vec()).unwrap();
        let re = Regex::new(r"(send/)(?P<value>[0-9]{1,3})").expect("Error creating regex string");
        // let re = Regex::new(r"(send/)(?P<value>[^ /]*)").unwrap();
        let caps = re.captures(&buffer)
            .expect("Error captureing regex");
        let mut state_locked = state.lock().expect(
            "Error locking state");

        let value = caps["value"].parse::<u32>()
            .expect("Error parsing request string to u8");

        let value = value as u8;

        match state_locked.byte_in(value){
            Ok(()) => (),
            Err(err) => {
                println!("Error: pushing byte {}", err)
            }
        }

        let contents = serde_json::to_vec(&*state_locked)
            .expect("Error converting to json");
        response = helper::make_json_header();
        response.extend(contents);
        
    } else if buffer.starts_with(image_original) {
        let contents = fs::read("image.png").expect("Error opening image.png");
        let length = contents.len();
        response = helper::make_image_header(length);
        response.extend(contents);
    } else if buffer.starts_with(image_current) {
        let contents = fs::read("current.png").unwrap_or_else(|_|{
            fs::read("image.png").unwrap()
        });
        let length = contents.len();
        response = helper::make_image_header(length);
        response.extend(contents);
    } else if buffer.starts_with(get_pixels) {
        response = helper::make_json_header();
        let mut state_locked = state.lock().unwrap();
        match state_locked.byte_out() {
            Ok(v) => {
                let d = color_table::Data::new(state_locked.cnt_out as u8, v);
                let contents = serde_json::to_vec(&d).unwrap();
                response.extend(contents);
            },
            Err(err) => {
                println!("Error retriving next value {}", err);
                let contents = serde_json::to_vec(&*state_locked).unwrap();
                response.extend(contents);
            }
        }
        // let contents = serde_json::to_vec(&*state_locked).unwrap();
        // response.extend(contents);
    } else if buffer.starts_with(start) {
        response = helper::make_json_header();
        let mut state_locked = state.lock().unwrap();
        state_locked.start();
        let contents = serde_json::to_vec(&*state_locked).unwrap();
        response.extend(contents);
    } else if buffer.starts_with(stop) {
        response = helper::make_json_header();
        let mut state_locked = state.lock().unwrap();
        state_locked.stop();
        let contents = serde_json::to_vec(&*state_locked).unwrap();
        response.extend(contents);
    } else if buffer.starts_with(reset) {
        response = helper::make_json_header();
        let mut state_locked = state.lock().unwrap();
        state_locked.reset();
        let contents = serde_json::to_vec(&*state_locked).unwrap();
        response.extend(contents);
    } else {
        response = helper::make_not_found_header();
       let contents = fs::read("404.html").unwrap();
        response.extend(contents); 
    }

    stream.write(response.as_ref()).unwrap();
    stream.flush().unwrap();
    
}

