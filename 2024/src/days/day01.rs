#[cfg(test)]
use crate::utils;

#[cfg(test)]
fn part1(input: &str) -> u32 {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                list1.push(num1);
                list2.push(num2);
            }
        }
    }

    let mut total = 0;
    let mut sorted_list1 = list1.clone();
    let mut sorted_list2 = list2.clone();
    sorted_list1.sort();
    sorted_list2.sort();

    for i in 0..sorted_list1.len() {
        let diff = if sorted_list1[i] > sorted_list2[i] {
            sorted_list1[i] - sorted_list2[i]
        } else {
            sorted_list2[i] - sorted_list1[i]
        };
        total += diff;
    }
    total
}

#[cfg(test)]
fn part2(input: &str) -> u32 {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                list1.push(num1);
                list2.push(num2);
            }
        }
    }

    let mut total = 0;
    for &num1 in &list1 {
        let count = list2.iter().filter(|&&x| x == num1).count();
        total += num1 * count as u32;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = utils::read_test_input(1);
        let input = utils::read_input(1);
        assert_eq!(part1(&test_input), 11);
        assert_eq!(part1(&input), 1765812);
    }

    #[test]
    fn test_part2() {
        let test_input = utils::read_test_input(1);
        let input = utils::read_input(1);
        assert_eq!(part2(&test_input), 31);
        assert_eq!(part2(&input), 20520794);
    }
}

// Some nice reddit solutions:

// helpful python utils inspiration: https://github.com/mcpower/adventofcode/blob/15ae109bc882ca688665f86e4ca2ba1770495bb4/utils.py

// ex 1: https://github.com/pkusensei/adventofcode2024/blob/6b9db1fb288619ae5f454cde9ad7918d39e7430e/d01/src/lib.rs

// ex 2:

// use counter::Counter;

// pub fn part_one(input: &str) -> Option<i32> {
//     let (mut col1, mut col2): (Vec<i32>, Vec<i32>) = input
//         .lines()
//         .filter_map(|line| {
//             let mut nums = line
//                 .split_whitespace()
//                 .filter_map(|x| x.parse::<i32>().ok());
//             Some((nums.next()?, nums.next()?))})
//         .unzip();

// col1.sort();
// col2.sort();

// let tot: i32 = col1.into_iter().zip(col2)
//                    .map(|l| (l.0 - l.1).abs()).sum();

// Some(tot)
// }

// pub fn part_two(input: &str) -> Option<usize> {
//     let (col1, col2): (Counter<_>, Counter<_>) = input
//         .lines()
//         .filter_map(|line| {
//             let mut nums = line
//                 .split_whitespace()
//                 .filter_map(|x| x.parse::<usize>().ok());
//             Some((nums.next()?, nums.next()?))})
//         .unzip();

//     let a: usize = col1.into_iter()
//            .map(|f| f.0 * f.1 * col2[&f.0]).sum();
//     Some(a)
// }
