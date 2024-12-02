#[cfg(test)]
use reqwest;
use std::env;
use std::fs;
use std::path::Path;

#[cfg(test)]
pub fn get_session() -> Result<String, Box<dyn std::error::Error>> {
    if let Ok(session) = env::var("AOC_SESSION") {
        println!("Session found in environment variable");
        return Ok(session);
    }

    let env_path = Path::new(".env");
    if env_path.exists() {
        let contents = fs::read_to_string(env_path)?;
        if let Some(session) = contents
            .lines()
            .find(|line| line.starts_with("AOC_SESSION="))
            .map(|line| line.trim_start_matches("AOC_SESSION=").to_string())
        {
            println!("Session found in .env file");
            return Ok(session);
        }
    }

    println!("Please enter your Advent of Code session cookie:");
    let mut session = String::new();
    std::io::stdin().read_line(&mut session)?;
    let session = session.trim().to_string();
    fs::write(env_path, format!("AOC_SESSION={}", session))?;

    Ok(session)
}

#[cfg(test)]
pub fn fetch_input(day: u32) -> Result<String, Box<dyn std::error::Error>> {    

    let file_path = format!("data/d{}.txt", day);
    if Path::new(&file_path).exists() {
        return Ok(fs::read_to_string(file_path)?);
    }

    let session = get_session()?;
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session))
        .send()
        .map_err(|e| format!("Failed to fetch input: {}", e))?;

    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch input: HTTP {} {}",
            response.status().as_u16(),
            response.status().canonical_reason().unwrap_or("")
        )
        .into());
    }

    let text = response
        .text()
        .map_err(|e| format!("Failed to read response text: {}", e))?;

    if text.trim().is_empty() {
        return Err("Received empty input from server".into());
    }

    fs::write(format!("data/d{}.txt", day), &text)
        .map_err(|e| format!("Failed to write input file: {}", e))?;
    println!("Day {} input saved in data/d{}.txt", day, day);
    Ok(text)
}

#[cfg(test)]
pub fn read_input(day: u32) -> String {
    fs::read_to_string(format!("data/d{}.txt", day))
        .expect("No data input file found")
}

#[cfg(test)]
pub fn read_test_input(day: u32) -> String {
    fs::read_to_string(format!("data/d{}_test.txt", day))
        .expect("No test input file found")
}
