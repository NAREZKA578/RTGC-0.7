use crossbeam_channel::{Receiver, Sender};
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
    active_jobs: Arc<AtomicBool>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, &'static str> {
        if size == 0 {
            return Err("Thread pool size must be greater than zero");
        }

        let (sender, receiver) = crossbeam_channel::unbounded();
        let receiver = Arc::new(Mutex::new(receiver));
        let active_jobs = Arc::new(AtomicBool::new(false));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), Arc::clone(&active_jobs)));
        }

        Ok(ThreadPool { workers, sender, active_jobs })
    }

    pub fn execute<F>(&self, f: F) -> JoinHandle
    where
        F: FnOnce() + Send + 'static,
    {
        self.active_jobs.store(true, Ordering::SeqCst);
        let (job_sender, job_receiver) = crossbeam_channel::bounded(1);
        let job = Box::new(move || {
            f();
            let _ = job_sender.send(());
        });
        self.sender.send(job).unwrap();
        JoinHandle { receiver: job_receiver }
    }

    pub fn wait_all(&self) {
        while self.active_jobs.load(Ordering::SeqCst) {
            thread::yield_now();
        }
    }
}

pub struct JoinHandle {
    receiver: Receiver<()>,
}

impl JoinHandle {
    pub fn join(self) -> Result<(), &'static str> {
        self.receiver.recv().map_err(|_| "Failed to join task")
    }
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>, active_jobs: Arc<AtomicBool>) -> Worker {
        let thread = thread::Builder::new()
            .name(format!("physics-worker-{}", id))
            .spawn(move || loop {
                let job = receiver.lock().recv();

                match job {
                    Ok(job) => {
                        job();
                    }
                    Err(_) => {
                        break;
                    }
                }
            })
            .unwrap();

        Worker { thread }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Wait for all active jobs to complete
        while self.active_jobs.load(Ordering::SeqCst) {
            thread::yield_now();
        }
        
        // Drop the sender to signal workers to stop
        drop(self.sender.clone());
        
        for worker in &mut self.workers {
            let _ = worker.thread.join();
        }
    }
}