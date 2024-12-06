#[cfg(test)]
use crate::utils;
#[cfg(test)]
use std::collections::HashMap;


#[cfg(test)]
fn part1(input: &String) -> i32 {
    let mut rules = Vec::new();
    let mut instructions = Vec::new();
    let mut parsing_rules = true;

    for line in input.lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            // Parse rules like "47|53" into tuple
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let from = parts[0].parse::<i32>().unwrap();
                let to = parts[1].parse::<i32>().unwrap();
                rules.push((from, to));
            }
        } else {
            // Parse instructions like "75,47,61,53,29" into Vec<i32>
            let nums: Vec<i32> = line
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            instructions.push(nums);
        }
    }    

    // println!("Rules: {:?}", rules);
    // println!("Instructions: {:?}", instructions);

    // go through the rules and make a lookup data structure for forwards rules. 
    // the key is the number, the value is a vector of numbers that must come after it
    let mut forwards = HashMap::new();
    for (from, to) in rules {
        forwards.entry(from).or_insert_with(Vec::new).push(to);
    }
    // println!("Forwards: {:?}", forwards);

    // go through the instructions, for each number, ensure that none of the numbers that must come after it have been seen before it in the row
    let mut total = 0;
    for instruction in instructions {
        let mut valid = true;
        for (i, &num) in instruction.iter().enumerate() {
            // Check if any numbers that must come after this one appear before it
            if let Some(must_follow) = forwards.get(&num) {
                for &required_after in must_follow {
                    // Look through previous numbers in this instruction
                    for &prev_num in instruction[..i].iter() {
                        if prev_num == required_after {
                            valid = false;
                            break;
                        }
                    }
                    if !valid {
                        break;
                    }
                }
            }
            if !valid {
                break;
            }
        }
        if valid {
            // Get middle entry and add to total
            let middle_index = instruction.len() / 2;
            total += instruction[middle_index];
            // println!("Valid instruction: {:?}, middle value: {}", instruction, instruction[middle_index]);
        }
    }
    println!("Total: {}", total);
    total
}

#[cfg(test)]
fn part2(input: &String) -> i32 {
    let mut rules = Vec::new();
    let mut instructions = Vec::new();
    let mut parsing_rules = true;

    for line in input.lines() {
        if line.is_empty() {
            parsing_rules = false;
            continue;
        }

        if parsing_rules {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() == 2 {
                let from = parts[0].parse::<i32>().unwrap();
                let to = parts[1].parse::<i32>().unwrap();
                rules.push((from, to));
            }
        } else {
            let nums: Vec<i32> = line
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect();
            instructions.push(nums);
        }
    }    
    let mut forwards = HashMap::new();
    for (from, to) in rules {
        forwards.entry(from).or_insert_with(Vec::new).push(to);
    }
    let mut total = 0;
    for instruction in instructions {
        let mut valid = true;
        for (i, &num) in instruction.iter().enumerate() {
            // Check if any numbers that must come after this one appear before it
            if let Some(must_follow) = forwards.get(&num) {
                for &required_after in must_follow {
                    // Look through previous numbers in this instruction
                    for &prev_num in instruction[..i].iter() {
                        if prev_num == required_after {
                            valid = false;
                            break;
                        }
                    }
                    if !valid {
                        break;
                    }
                }
            }
            if !valid {
                break;
            }
        }
        if !valid {
            // Create a mutable copy of the instruction that we can reorder
            let mut reordered = instruction.clone();
            
            // Keep trying to reorder until no more changes are needed
            let mut changes_made = true;
            while changes_made {
                changes_made = false;
                
                // Check each pair of numbers
                for i in 0..reordered.len() {
                    for j in i+1..reordered.len() {
                        let num1 = reordered[i];
                        let num2 = reordered[j];
                        
                        // If num1 must come after num2, swap them
                        if let Some(must_follow) = forwards.get(&num1) {
                            if must_follow.contains(&num2) {
                                reordered.swap(i, j);
                                changes_made = true;
                            }
                        }
                    }
                }
            }
            
            reordered.reverse();
            // println!("Reordered sequence: {:?}", reordered);
            let middle_idx = reordered.len() / 2;
            total += reordered[middle_idx];
        }
    }
    println!("Total: {}", total);
    total
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&utils::read_test_input(5)), 143);
        assert_eq!(part1(&utils::read_input(5)), 5091);        
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&utils::read_test_input(5)), 123);
        assert_eq!(part2(&utils::read_input(5)), 4681);        
    }
}
