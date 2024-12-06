use crate::utils;
use std::collections::HashSet;

pub fn solve() -> u32 {
    part1(&utils::read_test_input(6))
}

fn part1(input: &String) -> u32 {
    // read the input and store it as a 2d array
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    // padd the entire 2d array with "0", so first and last row and column are all "0"
    let mut padded_input = vec![vec!['0'; input[0].len() + 2]; input.len() + 2];
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            padded_input[i + 1][j + 1] = input[i][j];
        }
    }

    // Find start position using iterator methods instead of nested loops
    let start = padded_input.iter().enumerate()
        .find_map(|(i, row)| {
            row.iter().position(|&c| c == '^')
                .map(|j| (i, j))
        })
        .unwrap();
    padded_input[start.0][start.1] = '.';

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut position = start;
    let mut i = 0;
    let mut direction = directions[i];

    // Change the HashSet to only track positions, not directions
    let mut visited_positions: HashSet<(usize, usize)> = HashSet::with_capacity(input.len() * input[0].len());
    visited_positions.insert(position);  // Remove direction from insert

    loop {
        let new_char = check_new_position(&padded_input, position, direction);
        let new_position = (
            (position.0 as i32 + direction.0) as usize,
            (position.1 as i32 + direction.1) as usize
        );
        
        match new_char {
            '#' => {
                i += 1;
                direction = directions[i % 4];
            },
            '.' => {
                visited_positions.insert(new_position);
                position = new_position;
            },
            '0' => break,
            _ => unreachable!()
        }
    }

    visited_positions.len() as u32
}

fn part2(input: &String) -> u32 {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut padded_input = vec![vec!['0'; input[0].len() + 2]; input.len() + 2];
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            padded_input[i + 1][j + 1] = input[i][j];
        }
    }

    let start = padded_input.iter().enumerate()
        .find_map(|(i, row)| {
            row.iter().position(|&c| c == '^')
                .map(|j| (i, j))
        })
        .unwrap();
    padded_input[start.0][start.1] = '.';

    let canonical_path = get_canonical_path(&padded_input, start);
    
    // Use HashSet to ensure unique positions
    let unique_loop_positions: HashSet<(usize, usize)> = canonical_path.iter()
        .filter(|&&pos| pos != start)
        .filter(|&&pos| {
            let mut test_map = padded_input.clone();
            test_map[pos.0][pos.1] = '#';
            if has_loop(&test_map, start) {
                true
            } else {
                false
            }
        })
        .copied()
        .collect();

    unique_loop_positions.len() as u32
}

fn get_canonical_path(map: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<(usize, usize)> {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut path = Vec::new();
    let mut position = start;
    let mut i = 0;
    let mut direction = directions[i];

    path.push(position);

    loop {
        let new_char = check_new_position(&map, position, direction);
        let new_position = (
            (position.0 as i32 + direction.0) as usize,
            (position.1 as i32 + direction.1) as usize
        );
        
        match new_char {
            '#' => {
                i += 1;
                direction = directions[i % 4];
            },
            '.' => {
                position = new_position;
                path.push(position);
            },
            '0' => break,
            _ => unreachable!()
        }
    }
    path
}

fn has_loop(map: &Vec<Vec<char>>, start: (usize, usize)) -> bool {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut state_cache = HashSet::new();
    let mut position = start;
    let mut i = 0;
    let mut direction = directions[i];

    // A state is uniquely identified by (position, direction)
    while state_cache.insert((position, direction)) {
        let new_char = check_new_position(&map, position, direction);
        let new_position = (
            (position.0 as i32 + direction.0) as usize,
            (position.1 as i32 + direction.1) as usize
        );
        
        match new_char {
            '#' => {
                i += 1;
                direction = directions[i % 4];
            },
            '.' => position = new_position,
            '0' => return false,
            _ => unreachable!()
        }
    }
    true
}

// Move check_new_position outside of the other functions
fn check_new_position(padded_input: &Vec<Vec<char>>, position: (usize, usize), direction: (i32, i32)) -> char {
    let p = (position.0 as i32 + direction.0, position.1 as i32 + direction.1);
    padded_input[p.0 as usize][p.1 as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&utils::read_test_input(6)), 41);
        assert_eq!(part1(&utils::read_input(6)), 4903);
    }

    #[test]
    fn test_part2(){
        assert_eq!(part2(&utils::read_test_input(6)), 6);
        // assert_eq!(part2(&utils::read_input(6)), 1911);
    }
}
