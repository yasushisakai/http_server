
use std::time::SystemTime; 

#[derive(Serialize)]
pub struct State {
    is_running: bool,
    last_update: SystemTime,
    cnt_out: u32,
    cnt_in: u32,
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

}

