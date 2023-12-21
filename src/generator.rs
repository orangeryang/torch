use super::seeds::{generate_seed, get_seed};
use crate::U256 as u256;

pub struct CryptsAndCaverns {
    seed: u256,
    map: Vec<Vec<u8>>,
    name: String,
    enviroment: u8,
}

impl CryptsAndCaverns {
    pub fn new() -> CryptsAndCaverns {
        CryptsAndCaverns {
            seed: u256::zero(),
            map: Vec::new(),
            name: String::new(),
            enviroment: 0,
        }
    }

    pub fn select_or_generate_seed(&mut self, id: u32) {
        let seed: u256 = get_seed(id);

        self.seed = if seed != u256::zero() {
            seed
        } else {
            generate_seed(id)
        }
    }
}

pub fn generate_map(id: u32) -> CryptsAndCaverns {
    let mut cc = CryptsAndCaverns::new();

    cc.select_or_generate_seed(id);

    cc
}
