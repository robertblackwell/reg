extern crate wpool;
extern crate httpreader;

use  wpool::server::*;
use httpreader::HttpReader;

fn main() {

    let server = Server{
        m_nbr_workers   : 5,
        m_host          : "localhost".to_string(),
        m_port          :9001
    };
    server.listen();
}
