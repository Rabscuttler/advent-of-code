#[cfg(test)]
use crate::utils;

fn part1(input: &str) -> usize {
    use std::collections::{HashMap, BTreeMap};

    // Store frequency of each stone value instead of the full vector
    let mut stone_counts: BTreeMap<i64, usize> = BTreeMap::new();
    for num in input.split(" ").map(|s| s.parse::<i64>().unwrap()) {
        *stone_counts.entry(num).or_insert(0) += 1;
    }

    // Cache results of rules for each input value
    let mut rules_cache: HashMap<i64, Vec<i64>> = HashMap::new();

    // rules - returns new values without modifying output vector
    #[inline]
    fn rules(s: i64) -> Vec<i64> {
        if s == 0 {
            vec![1]
        } else if s >= 10 {
            let digits = (s as f64).log10().floor() as u32 + 1;
            
            if digits % 2 == 0 {
                let divisor = 10_i64.pow(digits/2);
                vec![s / divisor, s % divisor]
            } else {
                vec![s * 2024]
            }
        } else {
            vec![s * 2024]
        }
    }

    // Track seen states using the counts map
    let mut seen = HashMap::new();
    let mut i = 0;
    
    while i < 75 {
        // Convert current state to a hashable format
        if seen.insert(stone_counts.clone(), i).is_some() {
            break;
        }

        let mut next_counts: BTreeMap<i64, usize> = BTreeMap::new();
        
        // Process each unique stone value and its count
        for (&stone, &count) in stone_counts.iter() {
            let new_values = rules_cache.entry(stone)
                .or_insert_with(|| rules(stone));
                
            for &new_stone in new_values.iter() {
                *next_counts.entry(new_stone).or_insert(0) += count;
            }
        }

        stone_counts = next_counts;
        i += 1;
    }

    // Sum all counts to get total stones
    stone_counts.values().sum()
}

fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        // assert_eq!(part1(TEST_INPUT), 55312);
        // assert_eq!(part1(&utils::read_input(11)), 186203); // 25 times
       // assert_eq!(part1(&utils::read_input(11)), 221291560078593); // 75 times
    }


}