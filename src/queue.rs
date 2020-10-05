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
    mutex: Mutex<i32>,
    capacity: usize,
    list: RefCell::<LinkedList<i32>>,
}

impl Queue {
    pub fn new(capacity: usize) -> Queue {
        Queue {
            rdr_cvar: Condvar::new(),
            wrtr_cvar: Condvar::new(),
            mutex: Mutex::new(0),
            capacity: capacity,
            list: RefCell::new(LinkedList::<i32>::new())
        }

    }
    pub fn add(self: &Queue, sock: i32) {
        let q = self;
        let max:i32  = self.capacity.try_into().unwrap();
        let guard = self.mutex.lock().unwrap();
        let mut guard = self.wrtr_cvar.wait_while(guard, |n| *n > max).unwrap();

        println!("Queue::add  *guard {} \n", *guard);
        
        self.list.borrow_mut().push_back(sock);
        *guard += 1;

        if *guard > 0 {
            self.rdr_cvar.notify_one();
        }
        if *guard < max {
            q.wrtr_cvar.notify_one();
        }
    }
    pub fn remove(self: &Queue) -> i32 {
        let guard = self.mutex.lock().unwrap();
        let mut guard = self.rdr_cvar.wait_while(guard, |n| *n <= 0).unwrap();

        println!("Queue::remove guard  = {}\n", *guard);
        
        *guard -= 1;
        let result = self.list.borrow_mut().pop_front().unwrap();
        if *guard > 0 {
            self.rdr_cvar.notify_one();
        }
        if *guard < 10 {
            self.wrtr_cvar.notify_one();
        }
        return result;
    }
}

