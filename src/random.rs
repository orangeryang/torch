use std::{convert::TryInto, ops::Add};

use crate::{generator, seeds::generate_seed};

use super::U256 as u256;

pub(crate) fn random(seed: u256, min: u128, max: u128) -> u128 {
    if max == min {
        return min;
    }

    let mut input: Vec<u64> = vec![];

    let high: u128 = ((seed.0[3] as u128) << 64 + seed.0[2]).reverse_bits();
    let low: u128 = ((seed.0[1] as u128) << 64 + seed.0[0]).reverse_bits();

    let be_bytes = high.to_be_bytes();
    let (high_in_high, low_in_high) = be_bytes.split_at(64);

    input.push(u64::from_be_bytes(low_in_high.try_into().unwrap()));
    input.push(u64::from_be_bytes(high_in_high.try_into().unwrap()));

    let be_bytes = low.to_be_bytes();
    let (high_in_low, low_in_low) = be_bytes.split_at(64);

    input.push(u64::from_be_bytes(low_in_low.try_into().unwrap()));
    input.push(u64::from_be_bytes(high_in_low.try_into().unwrap()));

    println!("input: {:?}", input);

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
