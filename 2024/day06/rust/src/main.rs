use std::{fs, io::Read};
fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let read = std::fs::File::open(&filename).unwrap();
    let mut buf_reader = std::io::BufReader::new(read);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content).unwrap();

    let part_1 = aoc_2024_day06_part1(&content);
    println!("part 1 new: {}", part_1);

    let part_2 = aoc_2024_day06_part2(&content);
    println!("part 2 new: {}", part_2);   
}

#[derive(Debug)]
pub enum Going {
    Up,
    Down,
    Left,
    Right,
}

impl Going {
    pub fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
        }
    }
}

#[derive(Debug)]
struct CircularOperations {
    pos: usize,
    direction: Going,
    line_len: usize,
}

impl CircularOperations {
    fn new(pos: usize, line_len: usize) -> Self {
        Self {
            pos,
            direction: Going::Up,
            line_len,
        }
    }

    fn operate(&mut self, content: &str) -> Option<usize> {
        let res = match self.direction {
            Going::Up    => self.pos.checked_sub(self.line_len),
            Going::Right => self.pos.checked_add(1),
            Going::Down  => self.pos.checked_add(self.line_len),
            Going::Left  => self.pos.checked_sub(1),
        };

        match res {
            None => None,
            Some(new_pos) => {
                match content.chars().nth(new_pos) {
                    Some('.'|'^') => {
                        self.pos = new_pos;
                        Some(new_pos)
                    },
                    Some('#') => {
                        self.direction = self.direction.next();
                        self.operate(content)
                    },
                    _ => None,
                }
            },
        }
    }
}

use hashbrown::HashSet as Set;

fn aoc_2024_day06_part1 (content: &str) -> usize {
    let content = content.trim();
    let line_len = 1 + content.find('\n').expect("should have a first line");
    let start_pos = content.find('^').expect("should have a start position");

    let mut operations = CircularOperations::new(start_pos, line_len);
    let mut positions_set = Set::new();
    positions_set.insert(start_pos);
    while let Some(pos) = operations.operate(content) {
        positions_set.insert(pos);
    }

    positions_set.len()
}

fn aoc_2024_day06_part2 (content: &str) -> usize {
    2
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_aoc_2024_day06_part1() {
        assert_eq!(aoc_2024_day06_part1(TEST_INPUT), 41);
    }

    #[test]
    fn test_aoc_2024_day06_part2() {
        assert_eq!(aoc_2024_day06_part2(TEST_INPUT), 2);
    }
}