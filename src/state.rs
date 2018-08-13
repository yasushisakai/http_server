use image;
use chrono::prelude::*;
use std::time::{Duration, SystemTime, UNIX_EPOCH}; 
use super::helper::{convert_to_bytes, log, save_as_image};

#[derive(Serialize)]
pub struct State {
    is_running: bool,
    last_update: SystemTime,
    width: u32,
    height: u32,
    pub cnt_out: usize,
    pub cnt_in: usize,
    pub file_name: String,
    #[serde(skip_serializing)]
    image_bytes: Vec<u8>,
}

impl State {
   pub fn new (filename: &str) -> State {

       let img = match image::open("current") {
            Ok(img) => img,
            Err(_e) => {
                image::open(filename).expect("Error: couldn't open image.png")
            }
        };

       // converts dynamic image to rgb
        let img = img.to_rgb();

        let (w, h) = img.dimensions();
        let bytes = convert_to_bytes(&img);

    State{
        is_running: true,
        last_update: SystemTime::now(),
        width: w,
        height: h,
        cnt_out: 0,
        cnt_in: 0,
        file_name: filename.to_string(),
        image_bytes: bytes,
     }
   }

    pub fn start(&mut self) {
        self.is_running = true;
   }

   pub fn stop(&mut self) {
        self.is_running = false;
   }

    pub fn reset(&mut self) {
        self.cnt_in = 0;
        self.cnt_out = 0;
    }

    pub fn increment_in(&mut self){
        self.cnt_in += 1;
    }

    pub fn increment_out(&mut self){
            self.cnt_out += 1;
    }

    pub fn byte_in (&mut self, v: u8) -> Result<(), &'static str>{ 
        if self.is_running {
            self.cnt_in = self.cnt_in % (16 * 3);
            self.image_bytes[self.cnt_in] = v;

            let dt = Local::now();
            let log_line = format!("{}, in, {}, {}", dt.format("%s"),&self.cnt_in,&v);
            log(&log_line);
            println!("[{}] in:{}, {}",dt.format("%Y %m %d %H:%M:%S"),&self.cnt_in,&v);
            if self.last_update.elapsed().unwrap() > Duration::from_secs(20) {
                self.last_update = SystemTime::now();
                save_as_image(self.image_bytes.as_ref(), self.width, self.height);
            }
            self.increment_in();
            Ok(())
        } else {
            Err("cannot take any bytes, state is not running")
        }
    }

    pub fn byte_out (&mut self) -> Result<u8, &'static str> {
        if self.is_running {
            self.cnt_out = self.cnt_out % (16 * 3);
            let v = self.image_bytes[self.cnt_out];
            
            let dt = Local::now();
            let log_line = format!("{},out, {}, {}", dt.format("%s"),&self.cnt_out,&v);
            log(&log_line);
            println!("[{}]out:{}, {}",dt.format("%Y %m %d %H:%M:%S"),&self.cnt_out,&v);
            
            self.increment_out();
            Ok(v)
        } else {
            Err("cannot give you byte, state is not running")
        }
    }
}

