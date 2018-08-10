
use std::time::SystemTime; 

#[derive(Serialize)]
pub struct State {
    is_running: bool,
    last_update: SystemTime,
    cnt_out: usize,
    cnt_in: usize,
    image_bytes: Vec<u8>,
}

impl State {
   pub fn new () -> State {
    State{
        is_running: true,
        last_update: SystemTime::now(),
        cnt_out: 0,
        cnt_in: 0,
        image_bytes: Vec::new(),
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
            // make sure we don;t go beyond the length
            self.cnt_in = self.cnt_in % self.image_bytes.len();
            self.image_bytes[self.cnt_in] = v;
            self.increment_in();
            Ok(())
        } else {
            Err("cannot take any bytes, state is not running")
        }
    }

    pub fn byte_out (&mut self) -> Result<u8, &'static str> {
        if self.is_running {
            self.cnt_in = self.cnt_in % self.image_bytes.len();
            let v = self.image_bytes[self.cnt_out];
            self.increment_in();
            Ok(v)
        } else {
            Err("cannot give you byte, state is not running")
        }
    }

}

