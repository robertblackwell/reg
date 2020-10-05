/// All of this is now in another module now
use std::sync::{Arc, Condvar, Mutex};
use std::collections::LinkedList;
use std::convert::TryInto;
use std::cell::RefCell;

const NBR_WORKERS: usize = 5;
const QUEUE_MAX: usize = 5;

pub struct Queue {
    rdr_cvar: Condvar,
    wrtr_cvar: Condvar,
    mutex: Mutex<LinkedList<i32>>,
    capacity: usize,
    // list: RefCell::<LinkedList<i32>>,
}

impl Queue {
    pub fn new(capacity: usize) -> Queue {
        Queue {
            rdr_cvar: Condvar::new(),
            wrtr_cvar: Condvar::new(),
            mutex: Mutex::new(LinkList::<i32>::new()),
            capacity: capacity,
            // list: RefCell::new(LinkedList::<i32>::new())
        }

    }
    pub fn add(self: &Queue, sock: i32) {
        let q = self;
        let max:i32  = self.capacity.try_into().unwrap();
        let guard = self.mutex.lock().unwrap();
        let mut guard = self.wrtr_cvar.wait_while(guard, |list| *list.len() > max).unwrap();

        println!("Queue::add  *guard {} \n", *guard);
        
        *guard.push_back(sock);

        if *guard.len() > 0 {
            self.rdr_cvar.notify_one();
        }
        if *guard.len() < max {
            q.wrtr_cvar.notify_one();
        }
    }
    pub fn remove(self: &Queue) -> i32 {
        let guard = self.mutex.lock().unwrap();
        let mut guard = self.rdr_cvar.wait_while(guard, |list| *list.len() <= 0).unwrap();

        println!("Queue::remove guard  = {}\n", *guard);
        
        let result = *guard.pop_front().unwrap();
        
        if *guard.len() > 0 {
            self.rdr_cvar.notify_one();
        }
        if *guard.len() < 10 {
            self.wrtr_cvar.notify_one();
        }
        return result;
    }
}

