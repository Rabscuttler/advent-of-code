#[cfg(test)]
use crate::utils;

#[cfg(test)]
fn part1(input: &str) -> i32 {
    fn to_grid(input: &str) -> Vec<Vec<i32>> {
        input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect()
    }
    fn check_oob(i: i32, j: i32, grid: &Vec<Vec<i32>>) -> bool {
        i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32
    }
    // fn print_grid(grid: &Vec<Vec<i32>>) {
    //     for row in grid {
    //         println!("{:?}", row);
    //     }
    // }
    let grid = to_grid(input);
    // print_grid(&grid);

    // find all the zeroes, these are our trailheads
    let mut trailheads = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                trailheads.push((i, j));
            }
        }
    }
    // for each trailhead, find the number of 9s that can be reached
    // for each trailhead, add another constrained grid of the values that can be reached by going max 9 steps in any direction, as this is the max distance we can travel in 9 moves
    // output is a tuple of the trailhead coordinates, and the Vec constrained grid    
    
    fn create_trailheads_grids(trailheads: &Vec<(usize, usize)>, grid: &Vec<Vec<i32>>) -> Vec<((usize, usize), Vec<Vec<i32>>)> {
        let mut trailheads_grids = Vec::new();
        for t in trailheads {
            let mut trailhead_grid = Vec::new();
            for i in (t.0 as i32 - 9)..(t.0 as i32 + 10) {
                let mut row = Vec::new();
                for j in (t.1 as i32 - 9)..(t.1 as i32 + 10) {
                    if check_oob(i, j, grid) {
                        continue;
                    }
                    row.push(grid[i as usize][j as usize]);
                }
                if !row.is_empty() {
                    trailhead_grid.push(row);
                }
            }
            let center_i = 9.min(t.0);
            let center_j = 9.min(t.1);
            trailhead_grid[center_i][center_j] = 10;
            trailheads_grids.push((*t, trailhead_grid));
        }
        trailheads_grids
    }

    let trailheads_grids: Vec<((usize, usize), Vec<Vec<i32>>)> = create_trailheads_grids(&trailheads, &grid);

    // print trailheads_grids
    // for (t, g) in &trailheads_grids {
    //     println!("trailhead: {:?}", t);
    //     print_grid(&g);
    // }

    // reach a 9 from a 0, by traversing through 1,2,3,4,5,6,7,8 and 9
    // by moving left, right, up, down, in our grid. 
    
    fn count_reachable_nines(trailheads_grids: &Vec<((usize, usize), Vec<Vec<i32>>)>) -> i32 {
        let mut total_nines = 0;
        
        for (_trailhead, grid) in trailheads_grids {
            let mut visited = vec![vec![false; grid[0].len()]; grid.len()];        
            
            // Find all 9s and try to reach trailhead from each
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] == 9 {
                        let mut stack = Vec::new();
                        visited.iter_mut().for_each(|row| row.fill(false));
                        stack.push((i, j, 9)); // (i, j, current_value)
                        visited[i][j] = true;
                        
                        while let Some((i, j, current_value)) = stack.pop() {
                            if current_value == 10 {
                                total_nines += 1;
                                break;
                            }
                            
                            // Check all adjacent cells
                            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                            for (di, dj) in directions {
                                let ni = i as i32 + di;
                                let nj = j as i32 + dj;
                                
                                if ni >= 0 && ni < grid.len() as i32 && 
                                   nj >= 0 && nj < grid[0].len() as i32 {
                                    let ni = ni as usize;
                                    let nj = nj as usize;
                                    
                                    // Can only move to next number in sequence (going down)
                                    if !visited[ni][nj] && (
                                        (current_value == 9 && grid[ni][nj] == 8) ||
                                        (current_value == 8 && grid[ni][nj] == 7) ||
                                        (current_value == 7 && grid[ni][nj] == 6) ||
                                        (current_value == 6 && grid[ni][nj] == 5) ||
                                        (current_value == 5 && grid[ni][nj] == 4) ||
                                        (current_value == 4 && grid[ni][nj] == 3) ||
                                        (current_value == 3 && grid[ni][nj] == 2) ||
                                        (current_value == 2 && grid[ni][nj] == 1) ||
                                        (current_value == 1 && grid[ni][nj] == 10)
                                    ) {
                                        visited[ni][nj] = true;
                                        stack.push((ni, nj, grid[ni][nj]));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }        
        total_nines
    }
    
    count_reachable_nines(&trailheads_grids)
}

#[cfg(test)]
fn part2(input: &str) -> i32 {
    fn to_grid(input: &str) -> Vec<Vec<i32>> {
        input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect()
    }
    fn check_oob(i: i32, j: i32, grid: &Vec<Vec<i32>>) -> bool {
        i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32
    }
    // fn print_grid(grid: &Vec<Vec<i32>>) {
    //     for row in grid {
    //         println!("{:?}", row);
    //     }
    // }
    let grid = to_grid(input);
    // print_grid(&grid);

    // find all the zeroes, these are our trailheads
    let mut trailheads = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                trailheads.push((i, j));
            }
        }
    }
    // for each trailhead, find the number of 9s that can be reached
    // for each trailhead, add another constrained grid of the values that can be reached by going max 9 steps in any direction, as this is the max distance we can travel in 9 moves
    // output is a tuple of the trailhead coordinates, and the Vec constrained grid    
    
    fn create_trailheads_grids(trailheads: &Vec<(usize, usize)>, grid: &Vec<Vec<i32>>) -> Vec<((usize, usize), Vec<Vec<i32>>)> {
        let mut trailheads_grids = Vec::new();
        for t in trailheads {
            let mut trailhead_grid = Vec::new();
            for i in (t.0 as i32 - 9)..(t.0 as i32 + 10) {
                let mut row = Vec::new();
                for j in (t.1 as i32 - 9)..(t.1 as i32 + 10) {
                    if check_oob(i, j, grid) {
                        continue;
                    }
                    row.push(grid[i as usize][j as usize]);
                }
                if !row.is_empty() {
                    trailhead_grid.push(row);
                }
            }
            let center_i = 9.min(t.0);
            let center_j = 9.min(t.1);
            trailhead_grid[center_i][center_j] = 10;
            trailheads_grids.push((*t, trailhead_grid));
        }
        trailheads_grids
    }

    let trailheads_grids: Vec<((usize, usize), Vec<Vec<i32>>)> = create_trailheads_grids(&trailheads, &grid);

    // print trailheads_grids
    // for (t, g) in &trailheads_grids {
    //     println!("trailhead: {:?}", t);
    //     print_grid(&g);
    // }

    // reach a 9 from a 0, by traversing through 1,2,3,4,5,6,7,8 and 9
    // by moving left, right, up, down, in our grid. 
    
    fn count_number_of_valid_routes(trailheads_grids: &Vec<((usize, usize), Vec<Vec<i32>>)>) -> usize {
        let mut total_routes = 0;
        
        for (_trailhead, grid) in trailheads_grids {
            let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
            
            // Find all 9s and count paths from each to the 10
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] == 9 {
                        let mut paths = 0;
                        let mut stack = Vec::new();
                        visited.iter_mut().for_each(|row| row.fill(false));
                        stack.push((i, j, 9, Vec::new())); // (i, j, current_value, path)
                        visited[i][j] = true;
                        
                        while let Some((i, j, current_value, path)) = stack.pop() {
                            if current_value == 10 {
                                paths += 1;
                                continue;
                            }
                            
                            // Check all adjacent cells
                            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
                            for (di, dj) in directions {
                                let ni = i as i32 + di;
                                let nj = j as i32 + dj;
                                
                                if ni >= 0 && ni < grid.len() as i32 && 
                                   nj >= 0 && nj < grid[0].len() as i32 {
                                    let ni = ni as usize;
                                    let nj = nj as usize;
                                    
                                    // Can only move to next number in sequence (going down)
                                    if !visited[ni][nj] && (
                                        (current_value == 9 && grid[ni][nj] == 8) ||
                                        (current_value == 8 && grid[ni][nj] == 7) ||
                                        (current_value == 7 && grid[ni][nj] == 6) ||
                                        (current_value == 6 && grid[ni][nj] == 5) ||
                                        (current_value == 5 && grid[ni][nj] == 4) ||
                                        (current_value == 4 && grid[ni][nj] == 3) ||
                                        (current_value == 3 && grid[ni][nj] == 2) ||
                                        (current_value == 2 && grid[ni][nj] == 1) ||
                                        (current_value == 1 && grid[ni][nj] == 10)
                                    ) {
                                        visited[ni][nj] = true;
                                        let mut new_path = path.clone();
                                        new_path.push((ni, nj));
                                        stack.push((ni, nj, grid[ni][nj], new_path));
                                        visited[ni][nj] = false; // Backtrack to allow other paths
                                    }
                                }
                            }
                        }
                        total_routes += paths;
                    }
                }
            }
        }
        total_routes
    }
    
    count_number_of_valid_routes(&trailheads_grids).try_into().unwrap()
}

const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 36);
        // assert_eq!(part1(&utils::read_input(10)), 667); // Part 1: 667
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 81);
        // assert_eq!(part2(&utils::read_input(10)), 1344);
    }
}
