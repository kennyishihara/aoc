use std::io;

struct Solution<'a> {
    input: &'a str,
    part1: &'a str,
    part2: &'a str,
}

fn main() {
    println!("Which date?");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read stdin. Make sure it's strings separated by space");

    // let inputs: Vec<u32> = input.split_whitespace().map(|s| s.parse::<u32>().expect("Not an integer")).collect();

}

