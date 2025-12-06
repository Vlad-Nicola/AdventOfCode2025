use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp::{min, max};

fn do_day4() -> std::io::Result<(u64, u64)>
{
    let input_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input");
    let file = File::open(input_path)?;
    let mut buf_reader = BufReader::new(file);

    let mut count1: u64 = 0;
    let mut count2: u64 = 0;

    let mut binary_matrix: Vec<Vec<u8>> = Vec::new();
    binary_matrix.push(Vec::new());
    
    let mut buf: [u8; 1] = [0];
    while buf_reader.read_exact(&mut buf).is_ok() {
        if buf[0] == b'\n' {
            binary_matrix.push(Vec::new());
        } else if buf[0] == b'@' {
            binary_matrix.last_mut().unwrap().push(1);
        } else if buf[0] == b'.' {
            binary_matrix.last_mut().unwrap().push(0);
        }
    }
    binary_matrix.pop();

    let mut binary_matrix_copy = binary_matrix.clone();
    let mut keep_iterating = true;
    let mut first_iteration = true;

    while keep_iterating {
        keep_iterating = false;
        for (i, row) in binary_matrix.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if binary_matrix[i][j] == 0 {
                    continue;
                }
                let mut elem_neighbours = 0;
                for k in max(i as isize - 1, 0) as usize..=min(i + 1, binary_matrix.len() - 1) {
                    for l in max(j as isize - 1, 0) as usize..=min(j + 1, row.len() - 1) {
                        if binary_matrix[k][l] == 1 {
                            elem_neighbours += 1;
                        }
                    }
                }
                elem_neighbours -= 1;
                if elem_neighbours < 4 {
                    keep_iterating = true;
                    binary_matrix_copy[i][j] = 0;
                    count2 += 1;
                    if first_iteration {
                        count1 += 1;
                    }
                }
            }
        }
        first_iteration = false;
        binary_matrix = binary_matrix_copy.clone();
    }

    return Ok((count1, count2));
}

fn main() -> std::io::Result<()> {
    let res = do_day4()?;

    println!("Star1: Number of removable rolls at first iteration: {}", res.0);
    println!("Star2: Total number of rolls removed: {}", res.1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_result() {
        let result = do_day4().unwrap();
        assert_eq!(result.0, 1602);
        assert_eq!(result.1, 9518);
    }
}
