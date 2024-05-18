use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{collections::VecDeque};
use std::error::Error;

use super::errs::{MaxSizedError, GenResultIsNoneError};

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

pub struct PoolItems<'a,T> {
    value : Option<Vec<T>>,
    is_use : AtomicBool,
    command : &'a mut (dyn PoolCommander<T> + Sync + Send)
}

impl<'a,T> PoolItems<'a,T> {
    pub(super) fn new(value : Vec<T>, command : &'a mut (dyn PoolCommander<T> + Sync + Send)) -> Self {
        PoolItems {
            value : Some(value),
            is_use : AtomicBool::new(true),
            command
        }
    }

    pub fn get_value<'b>(&'b mut self) -> &'b mut [T] where 'a : 'b {
        let r : &'b mut [T] = self.value.as_mut().unwrap().as_mut_slice();
        r
    }

    pub fn dispose(&mut self) {
        let used = self.is_use.load(Ordering::Relaxed);
        self.is_use.store(false, Ordering::Relaxed);

        if used == true {
            let val = self.value.take();
            self.command.disposes(val.unwrap());
        }
    }

    pub fn restoration(&mut self) {
        let used = self.is_use.load(Ordering::Relaxed);
        self.is_use.store(false, Ordering::Relaxed);

        if used {
            let val = self.value.take();
            self.command.restorations(val.unwrap());
        }
    }
}

impl<'a,T> Drop for PoolItems<'a,T> {
    fn drop(&mut self) {
        if self.is_use.load(Ordering::Relaxed) == true {
            self.restoration()
        }
    }
}

pub(super) trait PoolCommander<T> {
    fn dispose(&mut self, item : T);
    fn disposes(&mut self, item : Vec<T>);
    fn restoration(&mut self, item : T);
    fn restorations(&mut self, item : Vec<T>);
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
    pub fn alloc_size(&self) -> usize {
        let g_lock = self.mutex_unit.lock().unwrap();
        let ret = self.alloc_size;
        drop(g_lock);

        ret
    }
    pub fn max_size(&self) -> usize {
        self.max_size
    }
    pub fn new_alloc_if_len_zeros(&mut self, ps : Vec<P>) ->Result<Vec<T>,Box<dyn Error>> {
        let g_lock = self.mutex_unit.lock().unwrap();
        let l = ps.len();
        for p in ps {
            if self.items.len() == 0 {
                if self.alloc_size < self.max_size {
                    let gen_item = (self.gen)(p);
                if gen_item.is_none() {
                    return Err(Box::new(GenResultIsNoneError));
                }
                self.items.push_back(gen_item.unwrap());
                self.alloc_size += 1;
                } else {
                    return Err(Box::new(MaxSizedError));
                }
            }
        }       
        let mut ret = Vec::new();

        for _ in 0..l {
            let i = self.items.pop_front().unwrap();
            ret.push(i);
        }

        drop(g_lock);
        Ok(ret)
    }
    #[inline]
    pub fn new_alloc_if_len_zero(&mut self, p : P) ->Result<T,Box<dyn Error>> {
        let v = vec![p];
        let mut r = self.new_alloc_if_len_zeros(v)?;
        Ok(r.pop().unwrap())
    }
    pub fn get(&mut self, param : P) -> Result<PoolItem<T>, Box<dyn Error>> {
        let item = self.new_alloc_if_len_zero(param)?;
        Ok(PoolItem::new(item,self))
    }
    pub fn gets(&mut self, params : Vec<P>) -> Result<PoolItems<T>, Box<dyn Error>> {
        let r = self.new_alloc_if_len_zeros(params)?;

        let ret = PoolItems::new(r, self);
        Ok(ret)
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

    fn disposes(&mut self, v : Vec<T>) {
        let _g_lock = self.mutex_unit.lock().unwrap();
        self.alloc_size -= v.len();
    }

    fn restorations(&mut self, mut items : Vec<T>) {
        let _g_lock = self.mutex_unit.lock().unwrap();
        let l = items.len();
        for _ in 0..l {
            self.items.push_back(items.pop().unwrap());
        }
    }
}


#[cfg(test)]
mod pool_tests {
    use std::error::Error;
    #[test]
    pub fn test_pool() -> Result<(), Box<dyn Error>> {
        let mut p = super::Pool::new(Box::new(|x : ()| {
            return Some(())
        }),5);

        {
        }

        Ok(())
    }
}