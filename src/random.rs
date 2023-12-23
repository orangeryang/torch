use super::U256 as u256;

pub(crate) fn random(seed: u256, min: u128, max: u128) -> u128 {
    if max == min {
        return min;
    }

    let mut input: Vec<u64> = vec![];

    input.push(seed.0[3].reverse_bits());
    input.push(seed.0[2].reverse_bits());
    input.push(seed.0[1].reverse_bits());
    input.push(seed.0[0].reverse_bits());

    input = add_padding(input);

    if input.len() % 17 != 0 {
        panic!(b"Invalid keccak input size");
    }
    let mut state = [0u64; 25];
    for chunk in input.chunks(17) {
        for (i, val) in chunk.iter().enumerate() {
            state[i] ^= val;
        }
        keccak::f1600(&mut state)
    }

    let mut result: u256 = u256::zero();

    result.0[0] = state[2];
    result.0[1] = state[3];
    result.0[2] = state[0];
    result.0[3] = state[1];

    println!("result {:#X}", result);
    print!(" {:#X}", result.0[3]);
    print!(" {:#X}", result.0[2]);
    print!(" {:#X}", result.0[1]);
    println!(" {:#X}", result.0[0]);

    (result % (max - min) + min).as_u128()
}

fn add_padding(mut input: Vec<u64>) -> Vec<u64> {
    let words_divisor: u32 = 17;
    let last_block_num_full_words = input.len() as u32 % words_divisor;
    let first_word_to_append = 1;
    if last_block_num_full_words == 16 {
        input.push(0x8000000000000000 + first_word_to_append);
        return input;
    }
    input.push(first_word_to_append);
    return finalize_padding(input, 16 - last_block_num_full_words);
}

fn finalize_padding(mut input: Vec<u64>, num_padding_words: u32) -> Vec<u64> {
    if (num_padding_words == 1) {
        input.push(0x8000000000000000);
        return input;
    }

    input.push(0);
    return finalize_padding(input, num_padding_words - 1);
}

#[test]
fn tttttt() {
    use std::str::FromStr;
    let seed: u256 = u256::from_str("0x6955a1583265848238e6e663de8b9fe272fa9d1c77395f8e32a233e23e65da0c").expect("ohhh");
    println!("seed {:#X}", seed);
    let min: u128 = 1;
    let max: u128 = 15;
    let result = random(seed, min, max);
    assert_eq!(result, 9);
}
