use std::ops::Add;

use crate::{generator, seeds::generate_seed};

use super::U256 as u256;

pub(crate) fn random(seed: u256, min: u128, max: u128) -> u128 {
    if max == min {
        return min;
    }

    let mut input: Vec<u64> = vec![];

    // let high: u128 = seed.0[3].into() << 64 + seed.0[2];
    // let low: u128 = seed.0[1].into() << 64 + seed.0[0];

    0
}

pub(crate) fn reverse(input: &str) -> String {
    let mut output = String::new();

    for c in input.chars().rev() {
        output.push(c);
    }

    output
}

#[test]
fn tttttt() {
    let seed = generate_seed();
    let min = 2;
    let max = 6;

    random(seed, min, max);
}
