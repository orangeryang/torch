extern crate keccak;
extern crate uint;

use uint::construct_uint;

use generator::CryptsAndCaverns;

mod generator;
mod random;
mod seeds;

construct_uint! {
    pub struct U256(4);
}

fn main() {
    println!("Hold your torch here!");
    loop {
        println!("- type a number to generate a Crypts and Caverns");
        println!("- type any characters to quit");
        let id: u32 = {
            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Strange input!");
            match input.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            }
        };
        let cc: CryptsAndCaverns = (if id < 1 || id > 9000 {
            println!("Oh, let's generate a random one...");
            seeds::generate_seed()
        } else {
            println!("Your input is: {}", id);
            seeds::get_seed(id)
        })
        .generate_map();
        println!("{:?}", &cc);
    }
}
