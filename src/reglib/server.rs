/// All of this is now in another module now
use reg::reglib::worker::*;

const NBR_WORKERS: i32 = 5;

pub struct Server 
{
    pub m_host        : String,
    pub m_port        : i16
}

impl Server 
{
    pub fn listen(&self)  
    {
        let mut workers: [Worker; NBR_WORKERS];
        println!("Server.listen nbr_workers: {} host:{}  port: {} ", NBR_WORKERS, self.m_host, self.m_port);

    }

}
