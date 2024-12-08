#[cfg(test)]
use crate::utils;

const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

// const TEST_INPUT_2: &str = "..........
// ..........
// ..........
// ....a.....
// ........a.
// .....a....
// ..........
// ..........
// ..........
// ..........";

pub fn part1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut grid = Vec::new();
    let mut unique_chars = std::collections::HashSet::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        for &c in &chars {
            if c != '.' {
                unique_chars.insert(c);
            }
        }
        grid.push(chars);
    }

    fn find_antinodes(
        a_i: usize,
        a_j: usize,
        b_i: usize,
        b_j: usize,
    ) -> Option<((usize, usize), Option<(usize, usize)>)> {
        let diff = (b_i as isize - a_i as isize, b_j as isize - a_j as isize);
        Some((
            (
                (a_i as isize - diff.0) as usize,
                (a_j as isize - diff.1) as usize,
            ),
            Some((
                (b_i as isize + diff.0) as usize,
                (b_j as isize + diff.1) as usize,
            )),
        ))
    }

    let char_positions = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c != '.')
                .map(move |(j, &c)| (c, (i, j)))
        })
        .fold(std::collections::HashMap::new(), |mut map, (c, pos)| {
            map.entry(c).or_insert_with(Vec::new).push(pos);
            map
        });

    let mut antinodes_set = std::collections::HashSet::new();
    for positions in char_positions.values() {
        for (i, &pos1) in positions.iter().enumerate() {
            positions
                .iter()
                .skip(i + 1)
                .filter_map(|&pos2| find_antinodes(pos1.0, pos1.1, pos2.0, pos2.1))
                .for_each(|(a, b)| {
                    antinodes_set.insert(a);
                    b.map(|b| antinodes_set.insert(b));
                });
        }
    }

    // println!("antinodes_set: {:?}", antinodes_set);
    let mut valid_antinodes = 0;
    for &(i, j) in &antinodes_set {
        if i < grid.len() && j < grid[0].len() {
            valid_antinodes += 1;
        }
    }
    println!("Number of valid antinodes: {}", valid_antinodes);

    // Print original grid
    // println!("Original grid:");
    // for row in &grid {
    //     for &c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    // println!("\nGrid with antinodes marked as #:");
    // for i in 0..grid.len() {
    //     for j in 0..grid[0].len() {
    //         if antinodes_set.contains(&(i, j)) {
    //             print!("#");
    //         } else {
    //             print!("{}", grid[i][j]);
    //         }
    //     }
    //     println!();
    // }

    valid_antinodes
}

pub fn part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut grid = Vec::new();
    let mut unique_chars = std::collections::HashSet::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        for &c in &chars {
            if c != '.' {
                unique_chars.insert(c);
            }
        }
        grid.push(chars);
    }

    fn find_antinodes(
        a_i: usize,
        a_j: usize,
        b_i: usize,
        b_j: usize,
        grid: &Vec<Vec<char>>,
    ) -> Vec<(usize, usize)> {
        let diff = (b_i as isize - a_i as isize, b_j as isize - a_j as isize);
        let mut antinodes = Vec::new();

        // Start from original points a and b
        let mut curr_a = (a_i as isize, a_j as isize);
        let mut curr_b = (b_i as isize, b_j as isize);

        // Keep going in both directions until we leave bounds
        loop {
            // Try moving backwards from a
            let next_a = (curr_a.0 - diff.0, curr_a.1 - diff.1);
            if next_a.0 >= 0 && next_a.1 >= 0 {
                antinodes.push((next_a.0 as usize, next_a.1 as usize));
                curr_a = next_a;
            } else {
                break;
            }
        }

        loop {
            // Try moving forwards from b
            let next_b = (curr_b.0 + diff.0, curr_b.1 + diff.1);
            if next_b.0 >= 0
                && (next_b.0 as usize) < grid.len()
                && next_b.1 >= 0
                && (next_b.1 as usize) < grid[0].len()
            {
                antinodes.push((next_b.0 as usize, next_b.1 as usize));
                curr_b = next_b;
            } else {
                break;
            }
        }

        antinodes
    }

    let char_positions = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &c)| c != '.')
                .map(move |(j, &c)| (c, (i, j)))
        })
        .fold(std::collections::HashMap::new(), |mut map, (c, pos)| {
            map.entry(c).or_insert_with(Vec::new).push(pos);
            map
        });

    let mut antinodes_set = std::collections::HashSet::new();
    for positions in char_positions.values() {
        for (i, &pos1) in positions.iter().enumerate() {
            positions.iter().skip(i + 1).for_each(|&pos2| {
                // Simply add all antinodes from the Vec to the set
                let antinodes = find_antinodes(pos1.0, pos1.1, pos2.0, pos2.1, &grid);
                antinodes.into_iter().for_each(|pos| {
                    antinodes_set.insert(pos);
                });
            });
        }
    }

    // println!("antinodes_set: {:?}", antinodes_set);
    let mut valid_antinodes = 0;

    // First, collect all positions of chars that appear multiple times
    let mut all_positions = std::collections::HashSet::new();
    for (_, positions) in char_positions.iter() {
        if positions.len() > 1 {
            positions.iter().for_each(|&(i, j)| {
                all_positions.insert((i, j));
            });
        }
    }

    // Then add all antinode positions
    all_positions.extend(antinodes_set.clone());

    // Finally, count all valid positions
    for &(i, j) in &all_positions {
        if i < grid.len() && j < grid[0].len() {
            valid_antinodes += 1;
        }
    }

    println!("Number of valid antinodes: {}", valid_antinodes);

    // println!("Original grid:");
    // for row in &grid {
    //     for &c in row {
    //         print!("{}", c);
    //     }
    //     println!();
    // }

    // println!("\nGrid with antinodes marked as #:");
    // for i in 0..grid.len() {
    //     for j in 0..grid[0].len() {
    //         if antinodes_set.contains(&(i, j)) {
    //             print!("#");
    //         } else {
    //             print!("{}", grid[i][j]);
    //         }
    //     }
    //     println!();
    // }

    valid_antinodes
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 14);
        assert_eq!(part1(&utils::read_input(8)), 291);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 34);
        assert_eq!(part2(&utils::read_input(8)), 1015);
    }
}
