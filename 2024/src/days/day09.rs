#[cfg(test)]
// use crate::utils;

fn part1(input: &str) -> i128 {
    let mut blocks: Vec<usize> = Vec::new();
    let mut counter = 0;

    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            let digit = c
                .to_digit(10)
                .unwrap_or_else(|| panic!("Invalid digit at position {}: '{}'", i, c))
                as usize;
            for _ in 0..digit {
                blocks.push(counter);
            }
            counter += 1;
        } else {
            let digit = c
                .to_digit(10)
                .unwrap_or_else(|| panic!("Invalid digit at position {}: '{}'", i, c))
                as usize;
            for _ in 0..digit {
                blocks.push(usize::MAX); // Using MAX to represent dots
            }
        }
    }

    fn sort_blocks(blocks: &[usize]) -> Vec<usize> {
        let mut sorted_blocks = Vec::new();
        let mut blocks_copy = blocks.to_vec();
        let non_dot_count = blocks.iter().filter(|&&x| x != usize::MAX).count();
        let original_len = blocks.len();

        let mut processed = 0;
        for &block in blocks {
            if processed >= non_dot_count {
                break;
            }

            if block != usize::MAX {
                sorted_blocks.push(block);
                processed += 1;
            } else {
                while blocks_copy.last() == Some(&usize::MAX) {
                    blocks_copy.pop();
                }
                sorted_blocks.push(blocks_copy.pop().unwrap());
                processed += 1;
            }
        }

        // Add remaining dots
        sorted_blocks.extend(vec![usize::MAX; original_len - non_dot_count]);
        sorted_blocks
    }

    let sorted = sort_blocks(&blocks);

    fn count_blocks(blocks: &[usize]) -> i128 {
        blocks
            .iter()
            .enumerate()
            .filter(|(_, &block)| block != usize::MAX)
            .map(|(i, &block)| (i as i128) * (block as i128))
            .sum()
    }

    count_blocks(&sorted)
}

fn part2(input: &str) -> i128 {
    let mut blocks: Vec<usize> = Vec::new();
    let mut counter = 0;

    // Parse input into blocks, same as before
    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            let digit = c
                .to_digit(10)
                .unwrap_or_else(|| panic!("Invalid digit at position {}: '{}'", i, c))
                as usize;
            for _ in 0..digit {
                blocks.push(counter);
            }
            counter += 1;
        } else {
            let digit = c
                .to_digit(10)
                .unwrap_or_else(|| panic!("Invalid digit at position {}: '{}'", i, c))
                as usize;
            for _ in 0..digit {
                blocks.push(usize::MAX);
            }
        }
    }

    // More efficient move_blocks_left implementation
    fn move_blocks_left(blocks: &mut Vec<usize>) {
        let max_id = *blocks
            .iter()
            .filter(|&&x| x != usize::MAX)
            .max()
            .unwrap_or(&0);

        // Process each file ID in descending order
        for current_id in (0..=max_id).rev() {
            // Find all positions of current file
            let mut start_idx = 0;
            let mut file_size = 0;
            let mut found = false;

            // Find first occurrence and size
            for (i, &block) in blocks.iter().enumerate() {
                if block == current_id {
                    if !found {
                        start_idx = i;
                        found = true;
                    }
                    file_size += 1;
                }
            }

            if file_size == 0 {
                continue;
            }

            // Find leftmost valid position
            let mut best_pos = start_idx;
            let mut current_pos = 0;
            let mut streak = 0;

            while current_pos < start_idx {
                if blocks[current_pos] == usize::MAX {
                    streak += 1;
                    if streak == file_size {
                        best_pos = current_pos - file_size + 1;
                        break;
                    }
                } else {
                    streak = 0;
                }
                current_pos += 1;
            }

            if best_pos < start_idx {
                // Clear old positions
                for i in 0..file_size {
                    blocks[start_idx + i] = usize::MAX;
                }
                // Place at new position
                for i in 0..file_size {
                    blocks[best_pos + i] = current_id;
                }
            }
        }
    }

    move_blocks_left(&mut blocks);

    // Calculate final score
    blocks
        .iter()
        .enumerate()
        .filter(|(_, &block)| block != usize::MAX)
        .map(|(i, &block)| (i as i128) * (block as i128))
        .sum()
}

const TEST_INPUT: &str = "2333133121414131402";

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1928);
        // assert_eq!(part1(&utils::read_input(9)), 6288707484810);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2858);
        // assert_eq!(part2(&utils::read_input(9)), 6311837662089);
    }
}
