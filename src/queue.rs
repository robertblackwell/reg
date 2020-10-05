/// All of this is now in another module now
use std::sync::{Arc, Condvar, Mutex};
use std::collections::LinkedList;

const NBR_WORKERS: usize = 5;
const QUEUE_MAX: usize = 5;

pub struct Queue {
    rdr_cvar: Condvar,
    wrtr_cvar: Condvar,
    mutex: Mutex<i32>,
    capacity: usize,
    list: LinkedList<i32>,
}

impl Queue {
    pub fn new(capacity: usize) -> Queue {
        Queue {
            rdr_cvar: Condvar::new(),
            wrtr_cvar: Condvar::new(),
            mutex: Mutex::new(0),
            capacity: capacity,
            list: LinkedList::<i32>::new()
        }

    }
    pub fn add(self: &Queue, sock: i32) {

    }
    pub fn remove(qq: &Queue) -> i32 {
        let mut  result: i32 = -1;
        // let q = qq;
        // let guard = q.mutex.lock().unwrap();
        // let mut guard = q.rdr_cvar.wait_while(guard, |n| *n <= 0).unwrap();

        // println!("{} Reader {} n_resource = {}\n", result, result, *guard);
        
        // *guard -= 1;

        // if *guard > 0 {
        //     q.rdr_cvar.notify_one();
        // }
        // if *guard < 10 {
        //     q.wrtr_cvar.notify_one();
        // }
        return result;
    }
}

