use std::io::Read;
fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let read = std::fs::File::open(&filename).unwrap();
    let mut buf_reader = std::io::BufReader::new(read);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap();

    let part_1 = aoc_2024_day04_part1(&content);
    println!("part 1 new: {}", part_1);

    let part_2 = aoc_2024_day04_part2(&content);
    println!("part 2 new: {}", part_2);   
}

fn count_xmas_in_line (line: &str) -> usize {
    line
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .fold(0, |count, window| {
            let word = window.iter().collect::<String>();
            count + if word == "XMAS" || word == "SAMX" { 1 } else { 0 }
        })
}

fn aoc_2024_day04_part1 (content: &str) -> usize {
    let content = content.trim();
    let num_of_cols = content.find("\n").expect("Should find \\n in content &str");

    let mut num_of_lines = 0;
    let mut total = content.lines().fold(0, |acc, line| {
        num_of_lines += 1;

        acc + count_xmas_in_line(line)
    });

    for col_number in 0..num_of_cols {
        let mut transposed_column = String::with_capacity(num_of_lines);
        for line_number in 0..num_of_lines {
            let ix = line_number * num_of_lines + line_number + col_number;
            let letter = &content[ix..=ix];
            transposed_column.push_str(letter);
        }

        total += count_xmas_in_line(&transposed_column);
    }

    let num_of_cols = num_of_cols + 1;
    let max_index = content.len();
    for (x_pos, x) in content.chars().into_iter().enumerate() {
        if x != 'X' { continue }

        let mut word_a = String::with_capacity(4);
        let mut word_b = String::with_capacity(4);
        let mut word_c = String::with_capacity(4);
        let mut word_d = String::with_capacity(4);
        
        for next_char in 1..=3 {
            let mut ix = 
                x_pos as isize - (num_of_cols * next_char) as isize - next_char as isize;

            if ix >= 0  {
                let ix = ix as usize;
                word_a.push_str(&content[ix..=ix]);
            }

            ix = x_pos as isize - (num_of_cols * next_char) as isize + next_char as isize;
            if ix >= 0  {
                let ix = ix as usize;
                word_b.push_str(&content[ix..=ix]);
            }

            let mut ix = x_pos + num_of_cols * next_char - next_char;
            if max_index > ix {
                word_c.push_str(&content[ix..=ix]);
            }

            ix = x_pos + num_of_cols * next_char + next_char;
            if max_index > ix {
                word_d.push_str(&content[ix..=ix]);
            }
        }

        total +=
            if word_a == "MAS" { 1 } else { 0 } +
            if word_b == "MAS" { 1 } else { 0 } +
            if word_c == "MAS" { 1 } else { 0 } +
            if word_d == "MAS" { 1 } else { 0 };

        // println!("a:{} b:{} c:{} d:{} ", word_a, word_b, word_c, word_d);
    }

    total
}

fn aoc_2024_day04_part2 (content: &str) -> usize {
    let content = content.trim();
    let num_of_cols = 1 + content.find("\n").expect("Should find \\n in content &str");
    let mut total = 0;

    content.chars().into_iter().enumerate().fold(0, |total, (x_pos, x)| {
        if x != 'A' { return total }

        let mut word_a = String::with_capacity(2);
        let mut word_b = String::with_capacity(2);
    
        let mut ix = x_pos as isize - num_of_cols as isize - 1;
        match content.chars().nth(ix as usize) {
            None => (), Some(c) => word_a.push(c)
        };

        ix = x_pos as isize - num_of_cols as isize + 1;
        match content.chars().nth(ix as usize) {
            None => (), Some(c) => word_b.push(c)
        };

        let mut ix = x_pos + num_of_cols - 1;
        match content.chars().nth(ix as usize) {
            None => (), Some(c) => word_b.push(c)
        };

        ix = x_pos + num_of_cols + 1;
        match content.chars().nth(ix as usize) {
            None => (), Some(c) => word_a.push(c)
        };

        if (word_a == "MS" || word_a == "SM") && (word_b == "MS" || word_b == "SM") {
            total + 1
        } else {
            total
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT1_A: &str =
r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
    const TEST_INPUT1_B: &str =
r#"
....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX"#;

    #[test]
    fn test_aoc_2024_day04_part1() {
        assert_eq!(aoc_2024_day04_part1(TEST_INPUT1_A), 18);
        assert_eq!(aoc_2024_day04_part1(TEST_INPUT1_B), 18);
    }

    const TEST_INPUT2_A: &str =
r#"
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."#;


    #[test]
    fn test_aoc_2024_day04_part2_a() {

        assert_eq!(aoc_2024_day04_part2(TEST_INPUT2_A), 9);
    }

    const TEST_INPUT2_B: &str =
r#"
M.S
.A.
M.S"#;

    #[test]
    fn test_aoc_2024_day04_part2_b() {
        assert_eq!(aoc_2024_day04_part2(TEST_INPUT2_B), 1);
    }

    mod count_xmas_in_line_tests {
        use super::super::count_xmas_in_line;
        #[test]
        fn count_xmas_in_line_with_1_xmas() {
            assert_eq!(count_xmas_in_line("XXXMASSSS"), 1);
            assert_eq!(count_xmas_in_line("..XMAS..."), 1);
        }
        
        
        #[test]
        fn count_xmas_in_line_with_1_samx() {
            assert_eq!(count_xmas_in_line("XXSAMXSSS"), 1);
            assert_eq!(count_xmas_in_line("..SAMX..."), 1);
        }
    }
}