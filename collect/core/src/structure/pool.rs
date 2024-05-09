use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{collections::VecDeque};
use std::error::Error;

use super::{MaxSizedError, GenResultIsNoneError};

pub struct PoolItem<'a,T> {
    value : Option<T>,
    is_use : AtomicBool,
    command : &'a mut (dyn PoolCommander<T> + Sync + Send)
}

impl<'a,T> PoolItem<'a,T> {
    pub(super) fn new(value : T, command : &'a mut (dyn PoolCommander<T> + Sync + Send)) -> Self {
        PoolItem {
            value : Some(value),
            is_use : AtomicBool::new(true),
            command
        }
    }

    pub fn get_value<'b>(&'b mut self) -> &'b mut T where 'a : 'b {
        let r :&'b mut T = self.value.as_mut().unwrap();
        r
    }

    pub fn dispose(&mut self) {
        let used = self.is_use.load(Ordering::Relaxed);
        self.is_use.store(false, Ordering::Relaxed);

        if used == true {
            let val = self.value.take();
            self.command.dispose(val.unwrap());
        }
    }

    pub fn restoration(&mut self) {
        let used = self.is_use.load(Ordering::Relaxed);
        self.is_use.store(false, Ordering::Relaxed);

        if used {
            let val = self.value.take();
            self.command.restoration(val.unwrap());
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
    items: VecDeque<T>,
    max_size : usize,
    alloc_size : usize,
    mutex_unit : Mutex<()>
}

unsafe impl<T,P> Sync for Pool<T,P> {}
unsafe impl<T,P> Send for Pool<T,P> {}

impl<T,P> Pool<T,P> {
    pub fn new(gen : Box<dyn Fn(P) -> Option<T>>, max_size : usize) -> Self {
        Pool {
            gen,
            items : VecDeque::new(),
            max_size : max_size,
            alloc_size : 0,
            mutex_unit : Mutex::new(())
        }
    }

    pub fn get(&mut self, param : P) -> Result<PoolItem<T>, Box<dyn Error>> {
        let g_lock = self.mutex_unit.lock().unwrap();

        if self.items.len() == 0 {
            if self.alloc_size < self.max_size {
                let gen_item = (self.gen)(param);
                if gen_item.is_none() {
                    return Err(Box::new(GenResultIsNoneError));
                }
                self.items.push_back(gen_item.unwrap());
                self.alloc_size += 1;
            } else {
                return Err(Box::new(MaxSizedError));
            }
        }

        let item = self.items.pop_front().unwrap();

        drop(g_lock);
        Ok(PoolItem::new(item,self))
    }
}

impl<T,P> PoolCommander<T> for Pool<T,P> {
    fn dispose(&mut self, _ : T) {
        let _g_lock = self.mutex_unit.lock().unwrap();
        self.alloc_size -= 1;
    }

    fn restoration(&mut self, item : T) {
        let _g_lock = self.mutex_unit.lock().unwrap();
        self.items.push_back(item);
    }
}


