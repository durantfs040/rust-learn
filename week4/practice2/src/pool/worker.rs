use std::thread::{JoinHandle, spawn};


#[derive(Default)]
pub struct Worker {
    pub id: u32,
    pub name: String,
    pub handle: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: u32, name: String) -> Self {
        Worker {
            id,
            name,
            handle: None,
        }
    }

    pub fn run<F>(&mut self, target: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.handle = Some(spawn(target));
    }

    pub fn is_finished(&self) -> bool {
        self.handle.as_ref().unwrap().is_finished()
    }

    // NOTE: this function takes ownership, think about why
    pub fn join(self) {
        self.handle.unwrap().join().unwrap();
    }
}
