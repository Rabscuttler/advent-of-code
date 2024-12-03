#[cfg(test)]
use crate::utils;
#[cfg(test)]
use regex::Regex;

#[cfg(test)]
fn part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    // Use captures to get the numbers from the capture groups
    re.captures_iter(input)
        .map(|cap| {
            // Extract numbers from capture groups 1 and 2
            let x = cap[1].parse::<u32>().unwrap();
            let y = cap[2].parse::<u32>().unwrap();
            x * y
        })
        .sum()
}

#[cfg(test)]
fn part2(input: &str) -> u32 {
    let mul_re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    let dont_do_re = Regex::new(r"don't\(\)[\s\S]*?do\(\)").unwrap();

    let cleaned_input = dont_do_re.replace_all(input, "");
    println!("cleaned_input: {}", cleaned_input);

    mul_re
        .captures_iter(&cleaned_input)
        .map(|cap| {
            let x = cap[1].parse::<u32>().unwrap();
            let y = cap[2].parse::<u32>().unwrap();
            x * y
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 161);
        assert_eq!(part1(&utils::read_input(3)), 190604937);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT_2), 48);
        assert_eq!(part2(&utils::read_input(3)), 82857512);
        ()
    }
}
