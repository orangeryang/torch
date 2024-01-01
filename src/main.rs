mod generator;
mod random;
mod seeds;

extern crate keccak;
extern crate uint;

use uint::construct_uint;
construct_uint! {
    pub struct U256(4);
}

use generator::{generate_map, CryptsAndCaverns};
use U256 as u256;

fn main() {
    println!("Hold your torch here!");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Strange input!");

    let id: u32 = input.trim().parse().expect("Give me a number!");
    let seed: u256 = if id < 1 || id > 9000 {
        println!("Oh, let's generate a random one...");
        seeds::generate_seed()
    } else {
        println!("Your input ID was: {}", &input);
        seeds::get_seed(id)
    };

    let cc: CryptsAndCaverns = generate_map(seed);
    println!("{}", &cc.name);
}
