extern crate http_server;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::time::{Duration, SystemTime};
use std::sync::{Arc, Mutex};

use http_server::ThreadPool;

mod laundry_view;
mod helper;

use laundry_view::{LaundryView, LaundryStatus};


#[derive(Serialize)]
struct State {
    last_query: Option<SystemTime>,
    laundry: Option<LaundryStatus>
}

impl State {
    
}


fn main (){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(10);

    let mut last_query = SystemTime::now();
    let mut view = Arc::new(Mutex::new(LaundryView::new().unwrap()));
    let mut state = Arc::new(Mutex::new(State{last_query: None, laundry: None}));

    // println!("{:?}", view.get_status());

    for stream in listener.incoming() {
        let stream = stream.unwrap(); 
        let view = Arc::clone(&view);
        let state = Arc::clone(&state);

        pool.execute(move || {
            handle_connection(stream, state, view);
        });
     }
}

fn handle_connection(mut stream: TcpStream, state: Arc<Mutex<State>>,view: Arc<Mutex<LaundryView>>){
    let mut buffer = [0; 512]; // 512 bytes
    stream.read(&mut buffer).unwrap(); 

    println!("Request: {}", String::from_utf8_lossy(&buffer));

    let get = b"GET /westgate/ HTTP/1.1\r\n";
    let status_json = b"GET /westgate/status HTTP/1.1\r\n";

    let ok_status = b"HTTP/1.1 200 OK\r\n";
    let not_found_status = b"HTTP/1/1 404 NOT FOUND\r\n";
    let rn = b"\r\n";
    let header_json = b"Content-Type:application/json;charset=UTF-8\r\n";
    
    let mut response = Vec::new();
    
    if buffer.starts_with(get){
        response.extend_from_slice(ok_status);
        response.extend_from_slice(rn);
        let contents = fs::read("hello.html").unwrap();
        response.extend(contents);
    } else if buffer.starts_with(status_json) {
        response.extend_from_slice(ok_status);
        response.extend_from_slice(header_json);
        response.extend_from_slice(rn);

        let mut view_locked = view.lock().unwrap();
        
        let mut state_locked = state.lock().unwrap();
        match state_locked.last_query {
            Some(last_time) => {
                // is it stale?
                if last_time.elapsed().unwrap() > Duration::from_secs(60 * 5) {
                    state_locked.last_query = Some(SystemTime::now());
                    view_locked.update().unwrap(); 
                    state_locked.laundry = Some(view_locked.get_status());
                } 
            },
            None => {
                state_locked.last_query = Some(SystemTime::now());
                view_locked.update().unwrap(); 
                state_locked.laundry = Some(view_locked.get_status());
            }
        }

        let contents = serde_json::to_vec(&*state_locked).unwrap();
        response.extend(contents);
    } else {
        response.extend_from_slice(not_found_status);
        response.extend_from_slice(rn);
        let contents = fs::read("404.html").unwrap();
        response.extend(contents); 
    }

    stream.write(response.as_ref()).unwrap();
    stream.flush().unwrap();
    
}




