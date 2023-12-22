use super::random::random;
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
}

pub fn generate_map(seed: u256) -> CryptsAndCaverns {
    let mut cc = CryptsAndCaverns::new();

    cc
}
