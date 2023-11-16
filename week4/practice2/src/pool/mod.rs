pub mod worker;

use std::cell::RefCell;
use std::collections::VecDeque;
use crate::pool::worker::Worker;

pub struct Pool {
    pub capacity: u32,
    pub workers: Vec<RefCell<worker::Worker>>,
    pub jobs: VecDeque<Box<dyn FnOnce() -> () + Send + 'static>>,
}

impl Pool {
    pub fn new(capacity: u32) -> Pool {
        Pool {
            capacity,
            workers: Vec::new(),
            jobs: VecDeque::new(),
        }
    }

    pub fn add<F>(&mut self, target: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.jobs.push_back(Box::new(target));
    }

    pub fn start(mut self) {
        for i in 0..self.capacity {
            self.workers.push(RefCell::new(Worker::new(i, format!("Worker {}", i))))
        }

        while !self.jobs.is_empty() {
            for worker in &self.workers {
                let mut worker = worker.borrow_mut();
                if worker.handle.is_none() || worker.is_finished() {
                    let job = self.jobs.pop_front().unwrap();
                    worker.run(job);
                    break;
                }
            }
        }

        for worker in self.workers {
            worker.take().join();
        }
    }
}
