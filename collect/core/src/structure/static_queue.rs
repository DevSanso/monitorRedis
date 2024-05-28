use std::collections::VecDeque;
use std::sync::{Arc,Mutex};
pub struct StaticQueue<T> where T : 'static {
    raw_q : VecDeque<T>,
}

impl<T> StaticQueue<T> {
    pub fn new() -> Arc<Mutex<Self>> {
        let ret =StaticQueue { 
            raw_q: VecDeque::new()  
        };

        Arc::new(Mutex::new(ret))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.raw_q.pop_front()
    }

    pub fn push(&mut self, data : T) {
        self.raw_q.push_back(data);
    }

    pub fn clear(&mut self) {
        self.raw_q.clear();
    }
}
