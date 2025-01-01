use super::super::utils;
use regex::Regex;

pub fn part1(path: &str) -> Result<u32, std::io::Error> {
    let buffer = utils::read_file_as_vec::<String>(path)?;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sol: u32 = buffer
        .iter()
        .flat_map(|line| {
            re.captures_iter(line).map(|cap| {
                let n1: u32 = cap[1].parse().unwrap();
                let n2: u32 = cap[2].parse().unwrap();
                n1 * n2
            })
        })
        .sum();
    Ok(sol)
}

pub fn part2(path: &str) -> Result<u32, std::io::Error> {
    let buffer = utils::read_file_as_vec::<String>(path)?;
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d+),(\d+)\)").unwrap();
    let (sum, _) = buffer
        .iter()
        .flat_map(|line| re.captures_iter(line))
        .fold((0, true), |(sum, flag), cap| {
            match &cap[0] {
                "don't()" => (sum, false),
                "do()" => (sum, true),
                _ => {
                    if flag {
                        if let (Some(n1), Some(n2)) = (cap.get(1), cap.get(2)) {
                            let n1: u32 = n1.as_str().parse().unwrap();
                            let n2: u32 = n2.as_str().parse().unwrap();
                            (sum + n1 * n2, flag)
                        } else {
                            (sum, flag)
                        }
                    } else {
                        (sum, flag)
                    }
                }
            }
        });
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part1("src/data/day3_test.txt")?, 161);
        Ok(())
    }

    #[test]
    fn test_day02() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part2("src/data/day3_part2_test.txt")?, 48);
        Ok(())
    }
}
