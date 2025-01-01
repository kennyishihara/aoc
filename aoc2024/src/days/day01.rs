use super::super::utils;

pub fn part1(path: &str) -> Result<u32, std::io::Error> {
    let buffer = utils::read_file_as_nested_vecs::<u32>(path)?;
    let mut left = buffer.iter().map(|l| l[0]).collect::<Vec<u32>>();
    let mut right = buffer.iter().map(|l| l[1]).collect::<Vec<u32>>();
    left.sort();
    right.sort();
    let sum = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    Ok(sum)
}

pub fn part2(path: &str) -> Result<u32, std::io::Error> {
    let buffer = utils::read_file_as_nested_vecs::<u32>(path)?;
    let left = buffer.iter().map(|l| l[0]).collect::<Vec<u32>>();
    let right = buffer.iter().map(|l| l[1]).collect::<Vec<u32>>();
    let sum: u32 = left
        .iter()
        .map(|l| right.iter().filter(|&&r| r == *l).count() as u32 * *l)
        .sum();
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part1("src/data/day1_test.txt")?, 11);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part2("src/data/day1_test.txt")?, 31);
        Ok(())
    }
}
