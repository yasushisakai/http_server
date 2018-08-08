use std::thread;
use std::sync::{Arc, Mutex, mpsc};

pub struct ThreadPool {
    workers : Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(s: usize) ->ThreadPool {
        assert!(s > 0);

        let (sender, receiver) = mpsc::channel();
    
        // Arc is something that lets you have multiple owners
        // Mutex is something that ensures only one worker gets a job from the receiver at a time
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(s);

        for i in 0..s {
           workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        ThreadPool{
            workers,
            sender
        }
    }

    pub fn execute<F>(&self, f:F)
        where
            F: FnOnce() + Send + 'static{
                let job = Box::new(f); 
                self.sender.send(Message::NewJob(job)).unwrap();
            }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap()
        }

        for worker in &mut self.workers{
            println!("Shutting down worker{}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    
    }
}

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl <F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

enum Message{
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnBox + Send + 'static>;

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(_id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {

        let id = _id;
        let thread =  thread::spawn(move ||{
            loop{
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                    Message::NewJob(job) => {
                    println!("worker {} go a job; executing.", id);
                    job.call_box();
                    },
                    Message::Terminate => {
                        println!("worker {} terminates", id);
                        break;
                    },
        }
            }
        });
        Worker{
            id,
            thread: Some(thread),
        }
    }
}

