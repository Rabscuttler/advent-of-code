use crate::utils;

pub fn solve() {
    let day = 1;
    let _ = utils::fetch_input(day);
    let input = utils::read_input(day);
    utils::print_results(day, &input, part1, part2);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_input = utils::read_test_input(1);
        assert_eq!(part1(&test_input), 11);
    }

    #[test]
    fn test_part2() {
        let test_input = utils::read_test_input(1);
        assert_eq!(part2(&test_input), 31);
    }
}
