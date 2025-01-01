use std::io;
use std::env;
mod days;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Which date?");
    let mut day_input = String::new();
    io::stdin()
        .read_line(&mut day_input)
        .expect("Failed to read the date.");
    let day = day_input.trim();

    println!("Which part?");
    let mut part_input = String::new();
    io::stdin()
        .read_line(&mut part_input)
        .expect("Failed to read the part.");
    let part = part_input.trim();
    let cwd = env::current_dir()?;
    let path = format!("{}/src/data/day{}.txt", cwd.display(), day);
    println!("Path: {}", path); 

    let result = match (day, part) {
        ("1", "1") => days::day01::part1(&path),
        ("1", "2") => days::day01::part2(&path),
        ("2", "1") => days::day02::part1(&path),
        ("2", "2") => days::day02::part2(&path),
        ("3", "1") => days::day03::part1(&path),
        ("3", "2") => days::day03::part2(&path),
        _ => {
            eprintln!("Invalid date or part specified.");
            return Err(format!("Invalid day '{}' or part '{}'.", day, part).into());
        }
    };

    match result {
        Ok(answer) => println!("Answer: {}", answer),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
