use std::fmt::Pointer;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::VecDeque;
use std::error::Error;

use crate::macros::utils_new_error_crate;

pub struct PoolItemOwned<T> {
    value : Option<T>,
    is_use : AtomicBool,
    command : Arc<dyn PoolCommander<T>>
}

impl<T> PoolItemOwned<T> {
    pub(super) fn new(value : T, command : Arc<dyn PoolCommander<T>>) -> Self {
        PoolItemOwned {
            value : Some(value),
            is_use : AtomicBool::new(true),
            command
        }
    }

    pub fn get_value<'b>(&'b mut self) -> &'b mut T {
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

impl<T> Drop for PoolItemOwned<T> {
    fn drop(&mut self) {
        if self.is_use.load(Ordering::Relaxed) == true {
            self.restoration()
        }
    }
}
pub(super) trait PoolCommander<T> {
    fn dispose(&self, item : T);
    #[warn(dead_code)]
    fn disposes(&self, item : Vec<T>);
    fn restoration(&self, item : T);
    #[warn(dead_code)]
    fn restorations(&self, item : Vec<T>);
}

struct OwnedPoolState<T> {
    items: VecDeque<T>,
    alloc_size : usize
}

pub struct OwnedPool<T,P> where T : 'static, P: 'static {
    gen : Box<dyn Fn(P) -> Option<T>>,
    max_size : usize,
    state : Mutex<OwnedPoolState<T>>,
    pool_name : String
}

unsafe impl<T,P> Sync for OwnedPool<T,P> {}
unsafe impl<T,P> Send for OwnedPool<T,P> {}

impl<T,P> OwnedPool<T,P> where T : 'static, P: 'static {
    pub fn new(name : String, gen : Box<dyn Fn(P) -> Option<T>>, max_size : usize) -> Arc<Self> {
        Arc::new(OwnedPool {
            gen,
            state : Mutex::new(OwnedPoolState { items: VecDeque::new(), alloc_size: 0 }),
            max_size: max_size,
            pool_name : name
        })
    }
    pub fn alloc_size(&self) -> usize {
        let g = self.state.lock().unwrap();
        let ret = g.alloc_size;
        drop(g);
        ret
    }
    pub fn max_size(&self) -> usize {
        self.max_size
    }

    fn new_alloc_if_len_zeros(&self, ps : Vec<P>) ->Result<Vec<T>,Box<dyn Error>> {
        let mut g = self.state.lock().unwrap();
        let l = ps.len();

        for p in ps {
            if g.items.len() < l {
                if g.alloc_size < self.max_size {
                    let gen_item = (self.gen)(p);
                if gen_item.is_none() {
                    return utils_new_error_crate!(proc, GenResultIsNoneError, self.pool_name.clone());
                }
                g.items.push_back(gen_item.unwrap());
                g.alloc_size += 1;
                } else {
                    return utils_new_error_crate!(proc, MaxSizedError, self.pool_name.clone());
                }
            }
        }   

        let mut ret = Vec::new();

        for _ in 0..l {
            let i = g.items.pop_front().unwrap();
            ret.push(i);
        }
        Ok(ret)
    }
    #[inline]
    fn new_alloc_if_len_zero(&self, p : P) ->Result<T,Box<dyn Error>> {
        let v = vec![p];
        let mut r = self.new_alloc_if_len_zeros(v)?;
        Ok(r.pop().unwrap())
    }

    pub fn get_owned(self : &Arc<Self>, param : P) -> Result<PoolItemOwned<T>, Box<dyn Error>> {
        let item = self.new_alloc_if_len_zero(param)?;
        Ok(PoolItemOwned::new(item, self.clone()))
    }
}

impl<T,P> PoolCommander<T> for OwnedPool<T,P> {
    fn dispose(&self, _ : T) {
        let mut g = self.state.lock().unwrap();
        g.alloc_size -= 1;
    }

    fn restoration(&self, item : T) {
        let mut g = self.state.lock().unwrap();
        g.items.push_back(item);
    }

    fn disposes(&self, v : Vec<T>) {
        let mut g = self.state.lock().unwrap();
        g.alloc_size -= v.len();
    }

    fn restorations(&self, mut items : Vec<T>) {
        let mut g = self.state.lock().unwrap();
        let l = g.items.len();
        for _ in 0..l {
            g.items.push_back(items.pop().unwrap());
        }
    }
}


#[cfg(test)]
mod pool_tests {
    use std::error::Error;
    #[test]
    pub fn test_pool_arc() -> Result<(), Box<dyn Error>> {
        use std::sync::Arc;
        use super::OwnedPool;

        let p : Arc<OwnedPool<(),()>> = OwnedPool::new(String::from("test"),Box::new(|_x : ()| {
            return Some(())
        }),5);

        {
            let a = p.get_owned(());
        }
        
        assert_eq!(1, p.alloc_size());

        {
            let mut a = p.get_owned(())?;
            a.dispose();
        }

        assert_eq!(0, p.alloc_size());

        Ok(()) 
    }
}