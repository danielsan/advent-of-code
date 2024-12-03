use std::io::Read;
fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let read = std::fs::File::open(&filename).unwrap();
    let mut buf_reader = std::io::BufReader::new(read);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap();

    let part_1 = aoc_2024_day02_part1(&content);
    println!("part 1 new: {}", part_1);

    let part_2 = aoc_2024_day02_part2(&content);
    println!("part 2 new: {}", part_2);   
}

fn process_line (line: &Vec<u8>) -> bool {
    let mut latest:u8 = 0;
    let mut started = false;
    let mut ascending = true;

    for (i, curr) in line.iter().enumerate() {
        // let curr = item.parse::<u8>().unwrap();
        if i == 0 {
            latest = *curr;
            continue;
        }

        if started {
            if ascending {
                if *curr <= latest {
                    return false;
                } else if (curr - latest) > 3 {
                    return false;
                }
            } else { // descending
                if *curr >= latest {
                    return false;
                } else if (latest - curr) > 3 {
                    return false;
                }
            }
            latest = *curr;
        } else {
            if *curr == latest {
                return false;
            }

            if curr.abs_diff(latest) > 3 {
                return false;
            }

            ascending = *curr > latest;
            latest = *curr;
            started = true;
        }
    }

    true
}

fn aoc_2024_day02_part1 (content: &str) -> u128 {
    content.trim().lines().fold(0 as u128, |acc, line| {
        let list: Vec<u8> = line.split_whitespace().map(|item| item.parse::<u8>().unwrap()).collect();
        if process_line(&list) {
            acc + 1
        } else {
            acc
        }
    })
}

fn aoc_2024_day02_part2 (content: &str) -> u128 {
    content.trim().lines().fold(0 as u128, |acc, line| {
        let list: Vec<u8> = line.split_whitespace().map(|item| item.parse::<u8>().unwrap()).collect();
        let list_len = list.len();

        if process_line(&list) {
            return acc + 1
        }

        for i in 0..list_len {
            let new_list = list.clone()
                .into_iter()
                .enumerate()
                .filter(|(idx, _)| *idx != i)
                .map(|(_, item)| item)
                .collect::<Vec<u8>>();

            if process_line(&new_list) {
                return acc + 1
            }
        }

        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_aoc_2024_day02_part2_new() {
        assert_eq!(aoc_2024_day02_part2(TEST_INPUT), 4);
    }

    const TEST_LIST_CUSTOM: &str =
r#"12 8 11 8 5
87 90 92 95 96 93
12 15 16 17 17"#;

    #[test]
    fn test_aoc_2024_b_day02_part2_new() {
        assert_eq!(aoc_2024_day02_part2(TEST_LIST_CUSTOM), 3);
    }
}