/// All of this is now in another module now
use std::sync::{Arc, Condvar, Mutex};
use std::collections::LinkedList;
use std::convert::TryInto;
use std::net::{TcpStream};

const NBR_WORKERS: usize = 5;
const QUEUE_MAX: usize = 5;

pub struct QueueEntry {
    stream: TcpStream
}

pub struct Queue {
    rdr_cvar: Condvar,
    wrtr_cvar: Condvar,
    mutex: Mutex<LinkedList<QueueEntry>>,
    capacity: usize,
    // list: RefCell::<LinkedList<i32>>,
}

impl Queue {
    pub fn new(capacity: usize) -> Queue {
        Queue {
            rdr_cvar: Condvar::new(),
            wrtr_cvar: Condvar::new(),
            mutex: Mutex::new(LinkedList::<QueueEntry>::new()),
            capacity: capacity,
            // list: RefCell::new(LinkedList::<i32>::new())
        }

    }
    pub fn add_stream(self: &Queue, strm: TcpStream)
    {
        let qentry = QueueEntry{
            stream: strm,
        };
        self.add(qentry);
    }

    pub fn add(self: &Queue, entry: QueueEntry) {
        let q = self;
        let max: usize  = self.capacity.try_into().unwrap();
        let guard = self.mutex.lock().unwrap();
        let mut guard = self.wrtr_cvar.wait_while(guard, |list| (*list).len() > max).unwrap();

        // println!("Queue::add  *guard {} \n", (*guard).len());
        (*guard).push_back(entry);

        if (*guard).len() > 0 {
            self.rdr_cvar.notify_one();
        }
        if (*guard).len() < max {
            q.wrtr_cvar.notify_one();
        }
    }
    pub fn remove(self: &Queue) -> TcpStream {
        let guard = self.mutex.lock().unwrap();
        let mut guard = self.rdr_cvar.wait_while(guard, |list| (*list).len() <= 0).unwrap();

        // println!("Queue::remove guard  = {}\n", (*guard).len());
        
        let result = (*guard).pop_front().unwrap().stream;
        
        if (*guard).len() > 0 {
            self.rdr_cvar.notify_one();
        }
        if (*guard).len() < 10 {
            self.wrtr_cvar.notify_one();
        }
        return result;
    }
}

