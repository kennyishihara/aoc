use super::super::utils;

pub fn part1(path: &str) -> Result<u32, std::io::Error> {
    let buffer = utils::read_file_to_vec::<u16>(path)?;
    let buffer_len = buffer.len();
    let mut safety_vec = vec![0; buffer_len];
    for n in 0..buffer_len {
        if check_safety(&buffer[n]) {
            safety_vec[n] = 1;
        }
    }
    let sol = safety_vec.iter().sum();
    Ok(sol)
}

pub fn part2(path: &str) -> Result<u32, std::io::Error> {
    let buffer = utils::read_file_to_vec::<u16>(path)?;
    let buffer_len = buffer.len();
    let mut safety_vec = vec![0; buffer_len];
    for n in 0..buffer_len {
        let safe = check_safety(&buffer[n]);
        if safe {
            safety_vec[n] = 1;
        } else {
            for m in 0..buffer[n].len() {
                let tmp: Vec<u16> = buffer[n][..m]
                        .iter()
                        .chain(&buffer[n][m + 1..])
                        .copied()
                        .collect();
                if check_safety(&tmp) {
                    safety_vec[n] = 1;
                    break;
                }
            }
        }
    }
    let sol = safety_vec.iter().sum();
    Ok(sol)
}

fn check_safety(buffer: &[u16]) -> bool {
    if buffer.windows(2).all(|x| x[0] < x[1]) || buffer.windows(2).all(|x| x[0] > x[1]) {
        if !buffer.windows(2).any(|x| x[1].abs_diff(x[0]) > 3) {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part1("src/data/day2_test.txt")?, 2);
        Ok(())
    }

    #[test]
    fn test_day02() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(part2("src/data/day2_test.txt")?, 4);
        Ok(())
    }
}
