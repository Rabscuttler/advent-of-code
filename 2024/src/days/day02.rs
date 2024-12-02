use crate::utils;

pub fn solve() {
    let day = 2;
    let _ = utils::fetch_input(day);
    let input = utils::read_input(day);
    utils::print_results(day, &input, part1, part2);
}

fn is_safe(v: &[i32]) -> bool {
    let mut prev = 0;
    let mut first = true;
    let mut sign = 0;
    let mut safe = true;
    for &p in v {
        if first {
            prev = p;
            first = false;
            continue;
        }
        let d = p - prev;
        if d == 0 || d.abs() > 3 {
            safe = false;
            break;
        }
        if sign == 0 {
            sign = d.signum();
        } else if sign != d.signum() {
            safe = false;
            break;
        }
        prev = p;
    }
    safe
}

fn part1(input: &str) -> u32 {
    // get difference between each pair
    // for each difference if difference = 0 or abs(difference) > 3 then fail
    // if sign is different between any differences then fail
    let mut count = 0;
    for line in input.lines() {
        let v: Vec<i32> = line.split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();        
        if is_safe(&v) {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> u32 {
    let mut count = 0;
    for line in input.lines() {    
        let v: Vec<i32> = line.split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect();    
        if is_safe(&v) {
            count += 1;
        } else {
            // try removing one number at a time and check if safe
            for i in 0..v.len() {
                let mut w = v.clone();
                w.remove(i);
                if is_safe(&w) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_INPUT), 2);
        assert_eq!(part1(&utils::read_input(2)), 534);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 4);
    }
}