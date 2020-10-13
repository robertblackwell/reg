/// All of this is now in another module now
use std::{thread, time};
use std::sync::{Arc};
use std::fmt;
use std::net::{TcpListener, TcpStream, IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::os::unix::io::{AsRawFd, IntoRawFd, FromRawFd};
use std::io::{Read, Write};

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

fn request_handler() {
    
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let response = "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-length: 43\r\nContent-type: text/html\r\n\r\n<html><body><h2>A Heading</h2><body></html>";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
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
                    let streamopt =  q.remove();
                    if let Some(stream) = streamopt {
                        println!("worker loop id: {} fd: {}", thread_id_2, stream.as_raw_fd());
                        // continue_flag = sock != -1;
                        handle_connection(stream);
                        std::thread::yield_now();
                        thread::sleep(time::Duration::from_secs(1));
                    } else {
                        break;
                    }
                }
                println!("worker loop id: {} exit", thread_id_2);
                return 0;
            }));
        };
        thread::sleep(time::Duration::from_secs(2));
        println!("Main thread before listen");

        let listener = TcpListener::bind((self.m_host.to_string(), self.m_port)).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            // let fd = stream.as_raw_fd();
            println!("Connection established! fd: {}", stream.as_raw_fd());
            let q = &*qdata;
            q.add_stream(stream);
        }
        // got here if 
        println!("Main thread after add");
        // forever loop - listening
        for ix in 0..self.m_nbr_workers {
            let q = &*qdata;
            q.add_terminate();
        }
        println!("Main thread before join");

        for handle in handles.into_iter() {
            handle.join().unwrap();
        }
    
    }
}
