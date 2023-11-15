use std::thread::JoinHandle;

#[allow(dead_code, unused_variables)]
#[derive(Debug)]
pub struct Worker {
    pub id: u32,
    pub name: String,
    pub handle: Option<JoinHandle<()>>,
}

impl Worker {
    #[allow(dead_code, unused_variables)]
    pub fn new(id: u32, name: String) -> Self {
        todo!()
    }

    #[allow(dead_code, unused_variables)]
    pub fn run<F>(&mut self, target: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // TODO: spawn thread
        todo!()
    }

    #[allow(dead_code, unused_variables)]
    pub fn is_finished(&self) -> bool {
        // TODO: check if thread is finished
        todo!()
    }

    // NOTE: this function takes ownership, think about why
    #[allow(dead_code, unused_variables)]
    pub fn join(self) {
        // TODO: join thread
        todo!()
    }
}
