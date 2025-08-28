pub fn square(s: u32) -> u64 {    
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    // 2^(s-1)
    (1..=s).map(|n| if n == s {1u64} else {0}).sum::<u64>() << (s-1)
}

pub fn total() -> u64 {
    //2^0 + 2^1 + 2^2 + ... +2^63
    // Equivalent to (2^64 - 1)
  (1..=64).map(square).sum()
}