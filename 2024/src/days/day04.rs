#[cfg(test)]
use crate::utils;

#[cfg(test)]
pub fn part1_3(input: &str) -> u32 {
    // read the input and store it as a 2d array
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut count = 0;
    let height = input.len();
    let width = input[0].len();

    // Define the 8 possible directions to check
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for i in 0..height {
        for j in 0..width {
            if input[i][j] == 'X' {
                // Check each direction from X
                for (di, dj) in directions.iter() {
                    let mut row = i as i32;
                    let mut col = j as i32;

                    // Look for M
                    row += di;
                    col += dj;
                    if row < 0 || row >= height as i32 || col < 0 || col >= width as i32 {
                        continue;
                    }
                    if input[row as usize][col as usize] != 'M' {
                        continue;
                    }

                    // Look for A
                    row += di;
                    col += dj;
                    if row < 0 || row >= height as i32 || col < 0 || col >= width as i32 {
                        continue;
                    }
                    if input[row as usize][col as usize] != 'A' {
                        continue;
                    }

                    // Look for S
                    row += di;
                    col += dj;
                    if row < 0 || row >= height as i32 || col < 0 || col >= width as i32 {
                        continue;
                    }
                    if input[row as usize][col as usize] != 'S' {
                        continue;
                    }

                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
    count
}

pub fn part2(input: &str) -> u32 {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = input.len();
    let width = input[0].len();

    let mut count = 0;

    // Define all four corners relative to center A
    let corners = [
        (-1, -1), // top-left
        (-1, 1),  // top-right
        (1, -1),  // bottom-left
        (1, 1),   // bottom-right
    ];

    for i in 0..height {
        for j in 0..width {
            if input[i][j] == 'A' {
                // Get all four corner characters
                let mut corner_chars = Vec::new();
                let mut valid_corners = true;

                for &(di, dj) in corners.iter() {
                    let row = i as i32 + di;
                    let col = j as i32 + dj;

                    if row < 0 || row >= height as i32 || col < 0 || col >= width as i32 {
                        valid_corners = false;
                        break;
                    }

                    corner_chars.push(input[row as usize][col as usize]);
                }

                if valid_corners {
                    // Check if both diagonals form S-M or M-S pairs
                    let diagonal1_valid = (corner_chars[0] == 'S' && corner_chars[3] == 'M')
                        || (corner_chars[0] == 'M' && corner_chars[3] == 'S');

                    let diagonal2_valid = (corner_chars[1] == 'S' && corner_chars[2] == 'M')
                        || (corner_chars[1] == 'M' && corner_chars[2] == 'S');

                    if diagonal1_valid && diagonal2_valid {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Final count: {}", count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "............
.MMMSXXMASM.
.MSAMXMSMSA.
.AMXSXMAAMM.
.MSAMASMSMX.
.XMASAMXAMM.
.XXAMMXXAMA.
.SMSMSASXSS.
.SAXAMASAAA.
.MAMMMXMMMM.
.MXMXAXMASX.
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1_3(&TEST_INPUT), 18);
        assert_eq!(part1_3(&utils::read_input(4)), 2378);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_INPUT), 9);
        assert_eq!(part2(&utils::read_input(4)), 1796);
        ()
    }
}
