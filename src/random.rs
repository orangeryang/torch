use super::U256 as u256;

pub(crate) fn random(seed: u256, min: u128, max: u128) -> u128 {
    if max == min {
        return min;
    }

    0
}
