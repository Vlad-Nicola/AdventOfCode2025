use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn find_max_joltage(line: &str, start: usize, count: usize) -> u64
{
    if count == 0 {
        return 0;
    }
    let mut max_joltage: u64 = 0;
    let mut max: char = '0';
    let mut max_index: usize = 0;
    let end = line.len() - count;
    for (i, c) in line.char_indices() {
        if i < start || i > end {
            continue;
        }
        if c > max {
            max = c;
            max_index = i;
        }
        if max == '9' {
            break;
        }
    }
    max_joltage += (10u64.pow(count as u32 - 1) * max.to_digit(10).unwrap() as u64
                + find_max_joltage(line, max_index + 1, count - 1)) as u64;
    max_joltage
}

fn do_day3() -> std::io::Result<(u64, u64)>
{
    let input_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input");
    let file = File::open(input_path)?;
    let mut buf_reader = BufReader::new(file);

    let mut count1: u64 = 0;
    let mut count2: u64 = 0;
    
    let mut line = String::new();
    while buf_reader.read_line(&mut line)? > 0 {
        line.truncate(line.trim_end().len());

        count1 += find_max_joltage(&line, 0, 2);
        count2 += find_max_joltage(&line, 0, 12);

        line.clear();
    }

    Ok((count1, count2))
}

fn main() -> std::io::Result<()>
{
    let res = do_day3()?;

    println!("Star1: Maximum joltage with 2 battery banks: {}", res.0);
    println!("Star2: Maximum joltage with 12 battery banks: {}", res.1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_result() {
        let result = do_day3().unwrap();
        assert_eq!(result.0, 17343);
        assert_eq!(result.1, 172664333119298);
    }
}
