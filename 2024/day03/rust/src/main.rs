use std::io::Read;
fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let read = std::fs::File::open(&filename).unwrap();
    let mut buf_reader = std::io::BufReader::new(read);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap();

    let part_1 = aoc_2024_day03_part1(&content);
    println!("part 1 new: {}", part_1);

    let part_2 = aoc_2024_day03_part2(&content);
    println!("part 2 new: {}", part_2);   
}

fn aoc_2024_day03_part1 (content: &str) -> u128 {
    let regexp = regex::Regex::new(r#"mul\((\d+),(\d+)\)"#)
        .expect("Regex should compile");

    regexp.captures_iter(content).fold(0, |acc, cap| {
        let a: u128 = cap[1].parse().expect("Should parse as a number");
        let b: u128 = cap[2].parse().expect("Should parse as a number");
        acc + a * b
    })
}

fn aoc_2024_day03_part2 (content: &str) -> u128 {
    let regexp = regex::Regex::new(r#"do(n't)?\(\)"#)
        .expect("Regex should compile");

    let mut lastest_end = 0;
    let mut total = 0u128;
    let mut calculate = true;
    // let locations = regexp.capture_locations(&content, s);
    while let Some(mat) = regexp.captures(&content[lastest_end..]) {
        let (start, end) = {
            let cap = mat.get(0).expect("Should have a match");
            (lastest_end + cap.start(), lastest_end + cap.end())
        };

        if calculate {
            total += aoc_2024_day03_part1(&content[lastest_end..start])
        }

        calculate = mat.get(1).is_none();

        lastest_end = end;
    };

    if calculate {
        total += aoc_2024_day03_part1(&content[lastest_end..])
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT1: &str =
    r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
    
    #[test]
    fn test_aoc_2024_day03_part1() {
        assert_eq!(aoc_2024_day03_part1(TEST_INPUT1), 161);
    }
    
    const TEST_INPUT2: &str = 
    r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
    #[test]
    fn test_aoc_2024_day03_part2() {
        assert_eq!(aoc_2024_day03_part2(TEST_INPUT2), 48);
    }
}