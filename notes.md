use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct Q {
    rdr_cvar: Condvar,
    wrtr_cvar: Condvar,
    mutex: Mutex<i32>,
}

impl Q {
    pub fn new() -> Q {
        Q {
            rdr_cvar: Condvar::new(),
            wrtr_cvar: Condvar::new(),
            mutex: Mutex::new(0),
        }
    }
}

fn writer(id: i32, qq: Arc<Q>) {
    let q = &*qq;
    for i in 0..10 {
        let guard = q.mutex.lock().unwrap();
        let mut guard = q.wrtr_cvar.wait_while(guard, |n| *n > 3).unwrap();

        println!("{}: Writer {} n_resource = {}\n", i, id, *guard);
        *guard += 1;

        if *guard > 0 {
            q.rdr_cvar.notify_one();
        }
        if *guard < 10 {
            q.wrtr_cvar.notify_one();
        }
    }
}

fn reader(id: i32, qq: Arc<Q>) {
    let q = &*qq;
    for i in 0..10 {
        let guard = q.mutex.lock().unwrap();
        let mut guard = q.rdr_cvar.wait_while(guard, |n| *n <= 0).unwrap();

        println!("{} Reader {} n_resource = {}\n", i, id, *guard);
        *guard -= 1;

        if *guard > 0 {
            q.rdr_cvar.notify_one();
        }
        if *guard < 10 {
            q.wrtr_cvar.notify_one();
        }
    }
}

fn main() {
    let data = Arc::new(Q::new());
    let data2 = data.clone();

    let t1 = thread::spawn(move || writer(0, data2));
    let t2 = thread::spawn(move || reader(1, data));

    t1.join().unwrap();
    t2.join().unwrap();
}
