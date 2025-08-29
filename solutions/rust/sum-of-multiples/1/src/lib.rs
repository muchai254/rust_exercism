pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    use std::collections::HashSet;

    let mut multiples = HashSet::new();

    for &factor in factors {
        if factor == 0 {
            continue; //skip 0 to avoid infinite loop
        }
        let mut multiple = factor;
        while multiple < limit {
            multiples.insert(multiple);
            multiple += factor;
        }
    }
    multiples.iter().sum()
}
