use std::sync::{Arc,Mutex};
use std::thread::{self, current};
use std::time::Duration;

use log;

use core::structure::static_queue::StaticQueue;

use crate::TItem;

type RawQ<T> = Arc<Mutex<StaticQueue<TItem<T>>>>;

pub struct TPool<T> where T : 'static + Send + Sync {
    wait_q : RawQ<T>,
    is_kill_thread : Arc<Mutex<bool>>
}

fn is_kill_flag_on(is_kill : &Arc<Mutex<bool>>) -> bool {
    let g = is_kill.lock().unwrap();
    *g
}

fn entry_point<T>(q : RawQ<T>, is_kill : Arc<Mutex<bool>>) where T : 'static + Send + Sync {
    loop {
        thread::sleep(Duration::from_millis(10));
        if is_kill_flag_on(&is_kill) { break; }

        let mut item : Option<TItem<T>> = None;
        {
            let mut g_q = q.lock().unwrap();
            item = g_q.pop();
        }

        if item.is_none() { continue; }
        
        let item_some = item.unwrap();

        let ret = item_some.1(item_some.0);   
        if let Err(x) = ret {
            log::error!("{} - error\n{}", current().name().unwrap(), x);
        }
    }
}

impl<T> TPool<T> where T : 'static + Send + Sync  {

    pub fn new(name : &'static str, alloc_size : usize) -> Self {
        let ret : TPool<T> = TPool {wait_q : StaticQueue::new(), is_kill_thread : Arc::new(Mutex::new(false))};
        
        for i in 0..alloc_size {
            let c_q = Arc::clone(&ret.wait_q);
            let c_kill = Arc::clone(&ret.is_kill_thread);
            
            let _ = thread::Builder::new()
                .name(format!("{}:{}",name, i))
                .stack_size(3 *  1024 * 1024)
            .spawn(|| {
                entry_point(c_q, c_kill);
            });
        }

        ret
    }
    pub fn use_pool(&mut self, item : TItem<T>) {
        if is_kill_flag_on(&self.is_kill_thread) {
            return;
        }
        let mut g = self.wait_q.lock().unwrap();
        g.push(item);
    }
    pub fn use_pool_from_vec(&mut self, mut items : Vec<TItem<T>>) {
        if is_kill_flag_on(&self.is_kill_thread) {
            return;
        }

        let mut g = self.wait_q.lock().unwrap();
        while let Some(s) = items.pop() {
            g.push(s);
        }
    }
}

impl<T> Drop for TPool<T> where T : 'static + Send + Sync {
    fn drop(&mut self) {
        let mut k_g = self.is_kill_thread.lock().unwrap();
        let mut q_g = self.wait_q.lock().unwrap();

        *k_g = false;
        q_g.clear();
    }
}

