use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}
type Job = Box<dyn FnOnce() + Send + 'static>;


impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // the channels will function as a queue of jobs, and its execution will send a job from the threadpool to the worker, which then sends the job to the thread
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{ workers, sender } 
    }

    /**
     * we can take closures as parameters with three different traits:
     * Fn, FnMut, FnOnce
     * thread::spawn takes FnOnce to execute the request one time
     * the F parameter also includes the trait-bound Send and the lifetime bound 'static
     * we need to Send to transfer the closure from one thread to another and 'static is necessary because 
     * we do not know how long it will take to execute
     * 
     *  */ 
     /*
    * we use () with FnOnce because it represents a closure that takes no params and returns the () type. 
    * The return type can be omitted from the signature but even if we have no params we need ()
    * 
    */ 
    pub fn execute<F>(&self, f: F) 
    where F: FnOnce() + Send + 'static, {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }

}

// structure for managing the process of waiting for code in a thread
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // closure the loop which asks the recieving end of the channel for a job and running the job when it gets one
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            // first we lock to acquire the mutex, and then we unwrap it to panic on any errors that come up. 
            // a lock mail fail in a "posioned state" which happens when another thread panicked while holding the lock rather than releasing. 
            // you could change this unwrap() to expect to set some other handler than panic
            // if we get the lock, we recv to recieve the jon from the channel and then unwrap again lol
            println!("Worker {} got a job; executing.", id);

            job();
        });

        Worker { id, thread }
    }
}