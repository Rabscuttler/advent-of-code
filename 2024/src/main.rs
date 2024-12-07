mod days;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!();
    println!("\x1b[1;32mWelcome to Advent of Code 2024!\x1b[0m");
    println!("Run tests for solutions");
    let _ = utils::fetch_input(7);
    // println!("{}", days::day07::part1(utils::read_input(7)));
    Ok(())
}
