use image;
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
    image_bytes: Vec<u8>,
}

impl State {
   pub fn new () -> State {

       let img = match image::open("current.png") {
            Ok(img) => img,
            Err(_e) => {
                image::open("image.png").expect("Error: couldn't open image.png")
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
        self.cnt_out +=1;
    }

    pub fn byte_in (&mut self, v: u8) -> Result<(), &'static str>{ 
        if self.is_running {
            self.cnt_in = self.cnt_in % self.image_bytes.len();
            self.image_bytes[self.cnt_in] = v;
            self.increment_in();
            
            if self.last_update.elapsed().unwrap() > Duration::from_secs(60) {
                    self.last_update = SystemTime::now();
                    save_as_image(self.image_bytes.as_ref(), self.width, self.height);
            }
            let time_since = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let log_in = format!("{},in,{},{}", time_since.as_secs(), self.cnt_in, v);
            log(log_in);
            Ok(())
        } else {
            Err("cannot take any bytes, state is not running")
        }
    }

    pub fn byte_out (&mut self) -> Result<u8, &'static str> {
        if self.is_running {
            self.cnt_in = self.cnt_in % self.image_bytes.len();
            let v = self.image_bytes[self.cnt_out];
            self.increment_out();
            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let log_out = format!("{}, out, {}, {}", now.as_secs(), self.cnt_out,v);
            log(log_out);
            Ok(v)
        } else {
            Err("cannot give you byte, state is not running")
        }
    }
}

