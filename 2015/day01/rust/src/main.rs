use std::io::Read;
fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let read = std::fs::File::open(&filename).unwrap();
    let mut buf_reader = std::io::BufReader::new(read);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap();

    let part_1 = aoc_2015_day01_part1(&content);
    println!("part 1 new: {}", part_1);

    let part_2 = aoc_2015_day01_part2(&content);
    println!("part 2 new: {}", part_2);   
}

pub fn aoc_2015_day01_part1 (content: &str) -> i128 {
    content.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

fn aoc_2015_day01_part2 (content: &str) -> i128 {
    let mut floor = 0;
    for (i, c) in content.char_indices() {
        match c {
            '(' => floor += 1,
            ')' => {
                if floor == 0 {
                    return (i + 1) as i128
                }
                floor -= 1;
            },
            _ => ()
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUTS1: [(&str, &str, i128); 5] = [
        ("(())",    "()()", 0),    // both result in floor 0.
        ("(((",     "(()(()(", 3), // both result in floor 3.
        ("))(((((", "(((", 3),     // also results in floor 3.
        ("())",     "))(", -1),    // both result in floor -1 (the first basement level).
        (")))", ")())())", -3),    // both result in floor -3.
    ];

    #[test]
    fn test_aoc_2015_day01_part1() {
        for (input1, input2, expected) in TEST_INPUTS1 {
            assert_eq!(aoc_2015_day01_part1(input1), expected);
            assert_eq!(aoc_2015_day01_part1(input2), expected);
        }
    }

    const TEST_INPUTS2: [(&str, i128); 5] = [
        (")",   1),
        ("())",   3),
        ("(()))", 5),
        ("((())))", 7),
        ("(((()))))", 9),
    ];

    #[test]
    fn test_aoc_2015_day01_part2_custom() {
        for (input, expected) in TEST_INPUTS2 {
            assert_eq!(aoc_2015_day01_part2(input), expected);
        }
    }
}