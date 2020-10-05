/// All of this is now in another module now
use std::{thread, time};
use std::sync::{Arc};

use crate::queue::{Queue};

const NBR_WORKERS: usize = 5;

pub struct Worker {
    pub w_1: String,
    pub w_2: i16,
    pub w_3: i16,
}

impl Worker {
    pub fn calculate_w(&self) -> i16 {
        self.w_2 * self.w_3
    }
    fn run(queue: Arc<Queue>) {
        while true {
            
        }
    }

}



pub struct Server 
{
    pub m_nbr_workers : usize,
    pub m_host        : String,
    pub m_port        : u16
}

impl Server 
{
    pub fn listen(&self)  
    {
        let qdata = Arc::new(Queue::new(self.m_nbr_workers));
        
        let mut workers: Vec<Worker> = Vec::with_capacity(self.m_nbr_workers);
        let mut handles: Vec<thread::JoinHandle<u64>> = Vec::with_capacity(self.m_nbr_workers);

        for thread_id in 0..(self.m_nbr_workers) {
            let qdata2 = qdata.clone();
            let thread_id_2 = thread_id;

            handles.push(thread::spawn(move || {
                let mut continue_flag = true;
                while continue_flag {
                    let q = &*qdata2;
                    let sock =  q.remove();
                    println!("worker loop id: {} sock: {}", thread_id_2, sock);
                    continue_flag = sock != -1;
                    std::thread::yield_now();
                    thread::sleep(time::Duration::from_secs(1));
                }
                println!("worker loop id: {} exit", thread_id_2);
                return 0;
            }));
        };
        
        thread::sleep(time::Duration::from_secs(2));
        println!("Main thread before add");
        for ix in 0..10 {
            let q = &*qdata;
            q.add(ix);
        }
        println!("Main thread after add");
        // forever loop - listening
        for ix in 0..self.m_nbr_workers {
            let q = &*qdata;
            q.add(-1);
        }
        println!("Main thread before join");

        for handle in handles.into_iter() {
            handle.join().unwrap();
        }
    
    }
}
