use crate::utils;

fn print_results(day: u32, test_input: &str, input: &str, part1_fn: fn(&str) -> u32, part2_fn: fn(&str) -> u32) {
    println!("\n\x1b[1;36m=== Day {} ===\x1b[0m", day);
    
    println!("\x1b[1;33mPart 1\x1b[0m");
    println!("  Test: \x1b[1;32m{}\x1b[0m", part1_fn(test_input));
    println!("  Solution: \x1b[1;32m{}\x1b[0m", part1_fn(input));
    
    println!("\x1b[1;33mPart 2\x1b[0m"); 
    println!("  Test: \x1b[1;32m{}\x1b[0m", part2_fn(test_input));
    println!("  Solution: \x1b[1;32m{}\x1b[0m", part2_fn(input));
}

pub fn solve() {
    let _ = utils::fetch_input(1);
    let test_input = utils::read_test_input(1);
    let input = utils::read_input(1);
    print_results(1, &test_input, &input, part1, part2);
}

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
