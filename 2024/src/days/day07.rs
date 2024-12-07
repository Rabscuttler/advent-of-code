#[cfg(test)]
use crate::utils;

fn part1(input: &str) -> i64 {
    
    fn find_combinations(total: i64, nums: Vec<i64>) -> i64 {
        let operators = vec!['+', '*'];
        
        fn evaluate(nums: &[i64], ops: &[char]) -> i64 {
            let mut result = nums[0];
            for (i, &op) in ops.iter().enumerate() {
                match op {
                    '+' => result += nums[i + 1],
                    '*' => result *= nums[i + 1],
                    _ => unreachable!(),
                }
            }
            result
        }

        let num_ops = nums.len() - 1;
        let total_combinations = 2_i64.pow(num_ops as u32); // 2^n combinations for n operators
        
        // Try all possible combinations
        for i in 0..total_combinations {
            let current_ops: Vec<char> = (0..num_ops)
                .map(|j| if (i & (1 << j)) == 0 { '+' } else { '*' })
                .collect();
                
            if evaluate(&nums, &current_ops) == total {
                // Build expression string
                let mut expr = format!("{}", nums[0]);
                for (i, op) in current_ops.iter().enumerate() {
                    expr.push_str(&format!(" {} {}", op, nums[i + 1]));
                }
                // println!("Found combination for {}: {} = {}", total, expr, total);
                return total;
            }
        }
        
        0
    }

    input.lines()
        .map(|line| {
            let (total, nums) = line.split_once(':').unwrap();
            let total = total.parse::<i64>().unwrap();
            let nums = nums.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            find_combinations(total, nums)
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    fn find_combinations(total: i64, nums: Vec<i64>) -> i64 {
        let operators = vec!['+', '*', '|'];  // Added concatenation operator
        
        fn evaluate(nums: &[i64], ops: &[char]) -> i64 {
            let mut result = nums[0];
            for (i, &op) in ops.iter().enumerate() {
                match op {
                    '+' => result += nums[i + 1],
                    '*' => result *= nums[i + 1],
                    '|' => {
                        // Convert both numbers to strings, concatenate, then parse back
                        let concat = format!("{}{}", result, nums[i + 1]);
                        result = concat.parse().unwrap();
                    },
                    _ => unreachable!(),
                }
            }
            result
        }

        let num_ops = nums.len() - 1;
        let total_combinations = 3_i64.pow(num_ops as u32); // Now 3^n combinations for n operators
        
        // Try all possible combinations
        for i in 0..total_combinations {
            let current_ops: Vec<char> = (0..num_ops)
                .map(|j| {
                    // Use base-3 to handle three operators
                    match (i / 3_i64.pow(j as u32)) % 3 {
                        0 => '+',
                        1 => '*',
                        2 => '|',
                        _ => unreachable!(),
                    }
                })
                .collect();
                
            if evaluate(&nums, &current_ops) == total {
                // Build expression string
                let mut expr = format!("{}", nums[0]);
                for (i, op) in current_ops.iter().enumerate() {
                    expr.push_str(&format!(" {} {}", op, nums[i + 1]));
                }
                // println!("Found combination for {}: {} = {}", total, expr, total);
                return total;
            }
        }
        
        0
    }

    input.lines()
        .map(|line| {
            let (total, nums) = line.split_once(':').unwrap();
            let total = total.parse::<i64>().unwrap();
            let nums = nums.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            find_combinations(total, nums)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;  // This brings part1 into scope

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3749);
        // assert_eq!(part1(&utils::read_input(7)), 7885693428401);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 11387);
        // assert_eq!(part2(&utils::read_input(7)), 348360680516005);
    }
}
