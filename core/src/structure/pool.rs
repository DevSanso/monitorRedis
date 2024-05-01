use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize,AtomicBool, Ordering};
use std::{collections::VecDeque, sync::Arc};
use std::error::Error;

use super::{MaxSizedError, GenResultIsNoneError};

pub struct PoolItem<'a,T> {
    value : Option<T>,
    is_use : AtomicBool,
    command : Arc<Mutex<&'a mut dyn PoolCommander<T>>>
}

impl<'a,T> PoolItem<'a,T> {
    pub(super) fn new(value : T, command : Arc<Mutex<&'a mut dyn PoolCommander<T>>>) -> Self {
        PoolItem {
            value : Some(value),
            is_use : AtomicBool::new(true),
            command
        }
    }

    pub fn get_value(&'a mut self) -> &'a mut T {
        let r = self.value.as_mut().unwrap();
        r
    }

    pub fn dispose(&mut self) {
        let used = self.is_use.load(Ordering::Relaxed);
        self.is_use.store(false, Ordering::Relaxed);

        if used == true {
            let mut mg = self.command.lock().unwrap();
            let val = self.value.take();
            mg.dispose(val.unwrap());
        }
    }

    pub fn restoration(&mut self) {
        let used = self.is_use.load(Ordering::Relaxed);
        self.is_use.store(false, Ordering::Relaxed);

        if used {
            let mut mg = self.command.lock().unwrap();
            let val = self.value.take();
            mg.restoration(val.unwrap());
        }
    }
}

impl<'a,T> Drop for PoolItem<'a,T> {
    fn drop(&mut self) {
        if self.is_use.load(Ordering::Relaxed) == true {
            self.restoration()
        }
        
    }
}

pub(super) trait PoolCommander<T> {
    fn dispose(&mut self, item : T);
    fn restoration(&mut self, item : T);
}

pub struct Pool<T,P> {
    gen : Box<dyn Fn(P) -> Option<T>>,
    items: Mutex<VecDeque<T>>,
    max_size : AtomicUsize,
    alloc_size : AtomicUsize
}

impl<T,P> Pool<T,P> {
    pub fn new(gen : Box<dyn Fn(P) -> Option<T>>, max_size : usize) -> Self {
        Pool {
            gen,
            items : Mutex::new(VecDeque::new()),
            max_size : AtomicUsize::new(max_size),
            alloc_size : AtomicUsize::new(0)
        }
    }

    pub fn get(&mut self, param : P) -> Result<PoolItem<T>, Box<dyn Error>> {
        let mut queue = self.items.lock().unwrap();

        if queue.len() == 0 {
            if self.alloc_size.load(Ordering::Relaxed) < self.max_size.load(Ordering::Relaxed) {
                let gen_item = (self.gen)(param);
                if gen_item.is_none() {
                    return Err(Box::new(GenResultIsNoneError));
                }
                queue.push_back(gen_item.unwrap());
                self.alloc_size.fetch_add(1, Ordering::Relaxed);
            } else {
                return Err(Box::new(MaxSizedError));
            }
        }
        let item = queue.pop_front().unwrap();

        drop(queue);

        let m : Mutex<&mut dyn PoolCommander<T>> = Mutex::new(self);
        let arc : Arc<Mutex<& mut dyn PoolCommander<T>>>  = Arc::new(m);
        
        Ok(PoolItem::new(item,arc))
    }
}

impl<T,P> PoolCommander<T> for Pool<T,P> {
    fn dispose(&mut self, _ : T) {
        self.alloc_size.fetch_sub(1, Ordering::Relaxed);
    }

    fn restoration(&mut self, item : T) {
        let mut queue = self.items.lock().unwrap();
       queue.push_back(item);
    }
}


