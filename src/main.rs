mod generator;
mod seeds;

use uint::construct_uint;

extern crate keccak;
extern crate uint;

construct_uint! {
    pub struct U256(4);
}

fn main() {
    println!("Hold your torch here!");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Strange input!");
    println!("Your input was: {}", &input);

    let id: u32 = input.trim().parse().expect("Give me a number!");
    let _cc = generator::generate_map(id);
}
