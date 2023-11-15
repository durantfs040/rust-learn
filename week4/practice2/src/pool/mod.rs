pub mod worker;

use std::cell::RefCell;
use std::collections::VecDeque;

#[allow(dead_code, unused_variables)]
pub struct Pool {
    pub capacity: u32,
    pub workers: Vec<RefCell<worker::Worker>>,
    // NOTE: jobs is a queue of closures that take no arguments and return nothing
    pub jobs: VecDeque<Box<dyn FnOnce() -> () + Send + 'static>>,
}

impl Pool {
    #[allow(dead_code, unused_variables)]
    pub fn new(capacity: u32) -> Pool {
        todo!()
    }

    #[allow(dead_code, unused_variables)]
    pub fn add<F>(&mut self, target: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: add job to jobs
        todo!()
    }

    #[allow(dead_code, unused_variables)]
    pub fn start(mut self) {
        // TODO: start workers
        todo!()
    }
}
