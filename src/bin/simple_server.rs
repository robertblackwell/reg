/// This is now an executable, technically is no longer part of the crate
/// so we have to call the library `src/lib.rs` as an external crate

extern crate reg;

use reg::reglib::server::*;
use reg::reglib::worker::*;
use reg::reglib::queue::*;

fn main() {

    let server = Server{
        m_nbr_workers   : 5,
        m_host          : "localhost".to_string(),
         m_port         :9001
    };
    server.listen();

    // let first = Server {
    //     s_1: "first one".to_string(),
    //     s_2: 2,
    //     s_3: -1,
    // };

    // let second = Worker {
    //     w_1: "second one".to_string(),
    //     w_2: 4,
    //     w_3: -2,
    // };

    // let third = Queue {
    //     q_1: "third one".to_string(),
    //     q_2: 8,
    //     q_3: -4,
    // };

    // println!("{}: {}, {} = {}", first.s_1, first.s_2, first.s_3, first.calculate_s());
    // println!("{}: {}, {} = {}", second.w_1, second.w_2, second.w_3, second.calculate_w());
    // println!("{}: {}, {} = {}", third.q_1, third.q_2, third.q_3, third.calculate_q());
}
