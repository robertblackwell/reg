/// All of this is now in another module now

pub struct Queue {
    pub q_1: String,
    pub q_2: i16,
    pub q_3: i16,
}

impl Queue {
    pub fn calculate_q(&self) -> i16 {
        self.q_2 * self.q_3
    }
}
