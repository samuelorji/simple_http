use std::sync::{Arc, LockResult, Mutex};
use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::thread::JoinHandle;

pub struct ThreadPool {
    workers : Vec<Worker>,
    sender : Sender<ThreadPoolMessage>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum ThreadPoolMessage{
    ProcessJob{job : Job},
    Terminate
}
struct Worker {
    id : u8,
    handle : Option<JoinHandle<()>>

}

impl Worker {
    fn new(id : u8, receiver : Arc<Mutex<Receiver<ThreadPoolMessage>>>) -> Self {
        let handle =  std::thread::spawn(move || {
            loop {

                let message = receiver.lock().unwrap().recv();
                match message {
                    Ok(ThreadPoolMessage::ProcessJob { job }) => {
                        println!("Worker {}, executing job", id);
                        job()
                    },
                    Ok(ThreadPoolMessage::Terminate) => {
                        println!("Terminating worker {}",id);
                        break;
                    },
                    Err(e) => {
                        println!("Receive Error: {:?}", e);
                        break
                    }
                }
            }
        });

        Worker{
            id,
            handle : Some(handle)
        }

    }

}
impl ThreadPool {
    pub fn new(num_threads: u8) -> ThreadPool {
        assert!(num_threads > 0);
        let mut workers = vec![];
        let (sender,receiver) = std::sync::mpsc::channel();
        let rec_arc = std::sync::Arc::new(Mutex::new(receiver));
        for id in 0..num_threads {
            workers.push(Worker::new(id,rec_arc.clone()))
        }
        ThreadPool {
            workers,
            sender
        }

    }

    pub fn execute<F : FnOnce() + Send + 'static>(&self, job : F) {
        self.sender.send(ThreadPoolMessage::ProcessJob {job : Box::new(job)}).unwrap()

    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(ThreadPoolMessage::Terminate);
        }

        for _worker in &mut self.workers {
            if let Some(worker) =  _worker.handle.take() {
                worker.join().unwrap()
            }
        }
    }
}