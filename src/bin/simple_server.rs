/// This is now an executable, technically is no longer part of the crate
/// so we have to call the library `src/lib.rs` as an external crate

extern crate reg;

use reg::server::*;

fn main() {

    let server = Server{
        m_nbr_workers   : 5,
        m_host          : "localhost".to_string(),
        m_port          :9001
    };
    server.listen();
}
