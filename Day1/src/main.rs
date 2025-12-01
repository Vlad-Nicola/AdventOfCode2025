use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn do_day1() -> std::io::Result<(i32, i32)>
{
    let file = File::open("input")?;
    let mut buf_reader = BufReader::new(file);

    let mut pos: i32 = 50;
    let mut count1: i32 = 0;
    let mut count2: i32 = 0;
    
    let mut line = String::new();
    while buf_reader.read_line(&mut line)? > 0 {
        let mut offset: i32 = line[1..].trim().parse().unwrap();
        if line.starts_with('L') {
            offset = -offset;
        }

        let init_pos = pos;
        pos += offset;

        if pos == 0 {
            count2 += 1;
        } else {
            count2 += pos.abs() / 100;
            if pos < 0 && init_pos != 0 {
                count2 += 1;
            }
        }

        pos %= 100;
        if pos == 0 {
            count1 += 1;
        } else if pos < 0 {
            pos += 100;
        }

        line.clear();
    }

    return Ok((count1, count2));
}

fn main() -> std::io::Result<()> {
    let res = do_day1()?;

    println!("Star1: Number of times at position 0: {}", res.0);
    println!("Star2: Number of times touching position 0: {}", res.1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_result() {
        let result = do_day1().unwrap();
        assert_eq!(result.0, 1066);
        assert_eq!(result.1, 6223);
    }
}
