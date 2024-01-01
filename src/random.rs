use super::U256 as u256;

pub(crate) fn random(seed: u256, min: u32, max: u32) -> u32 {
    if max == min {
        return min;
    }

    let mut input: Vec<u64> = vec![];

    input.push(reverse_bytes_u64(seed.0[3]));
    input.push(reverse_bytes_u64(seed.0[2]));
    input.push(reverse_bytes_u64(seed.0[1]));
    input.push(reverse_bytes_u64(seed.0[0]));

    input = add_padding(input);

    if input.len() % 17 != 0 {
        panic!("Invalid keccak input size");
    }
    let mut state = [0u64; 25];
    for chunk in input.chunks(17) {
        for (i, val) in chunk.iter().enumerate() {
            state[i] ^= val;
        }
        keccak::f1600(&mut state)
    }

    let mut result: u256 = u256::zero();

    result.0[0] = reverse_bytes_u64(state[3]);
    result.0[1] = reverse_bytes_u64(state[2]);
    result.0[2] = reverse_bytes_u64(state[1]);
    result.0[3] = reverse_bytes_u64(state[0]);

    println!("result {:#X}", result);

    (result % (max - min) + min).as_u32()
}

fn reverse_bytes_u64(input: u64) -> u64 {
    let mut input_str = format!("{:b}", input).to_string();
    if input_str.len() != 64 {
        for _i in 0..(64 - input_str.len()) {
            input_str = format!("0{}", input_str);
        }
    }

    let mut output_str = String::new();
    let mut temp: Vec<String> = Vec::new();
    let mut temp_str = String::new();
    for b in input_str.chars() {
        temp_str += &b.to_string();
        if temp_str.len() == 8 {
            temp.push(temp_str);
            temp_str = String::new();
        }
    }

    for v in temp.iter().rev() {
        output_str += v;
    }
    u64::from_str_radix(&output_str, 2).unwrap()
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
    if num_padding_words == 1 {
        input.push(0x8000000000000000);
        return input;
    }

    input.push(0);
    return finalize_padding(input, num_padding_words - 1);
}

#[test]
// #[ignore]
fn test_it() {
    use std::str::FromStr;
    let seed: u256 = u256::from_str("0x6955a1583265848238e6e663de8b9fe272fa9d1c77395f8e32a233e23e65da0c").expect("ohhh");
    // println!("seed {:#X}", seed);
    let min = 1;
    let max = 15;
    let result = random(seed, min, max);
    assert_eq!(result, 9);

    let a: u64 = 0xc3d781a79a156011;
    let b: u64 = 0x1160159AA781D7C3;
    assert_eq!(format!("{:#X}", b), format!("{:#X}", reverse_bytes_u64(a)));
}