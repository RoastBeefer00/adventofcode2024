use anyhow::Result;
use std::fs::read_to_string;
use nom::{
    character::complete::{anychar, char, i32, line_ending, one_of, space0}, combinator::opt, multi::many1, sequence::{preceded, terminated}, IResult
};

// fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
//     let (input, line) = take_till(line_ending)(input)?;
//     Ok((input, line.chars().collect()))
// }

fn get_all_lines(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(terminated(many1(one_of("XMAS")), line_ending))(input)
}

fn get_char_at_coordinates(input: &Vec<Vec<char>>, x: usize, y: usize) -> Option<char> {
    if let Some(row) = input.get(x) {
        if let Some(c) = row.get(y) {
            return Some(c.clone());
        }
    }

    None
}

fn count_xmas_horizontal(input: Vec<char>) -> i32 {
    // let mut i = input.into_iter();
    let mut count = 0;
    // while let Some(c) = i.next() {
    for i in 0..input.len() {
        if let Some(c) = input.get(i) {
            if c.clone() == 'X' {
                if let Some(m) = input.get(i+1) {
                    if m.clone() == 'M' {
                        if let Some(a) = input.get(i+2) {
                            if a.clone() == 'A' {
                                if let Some(s) = input.get(i+3) {
                                    if s.clone() == 'S' {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if c.clone() == 'S' {
                if let Some(a) = input.get(i+1) {
                    if a.clone() == 'A' {
                        if let Some(m) = input.get(i+2) {
                            if m.clone() == 'M' {
                                if let Some(x) = input.get(i+3) {
                                    if x.clone() == 'X' {
                                        count += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

fn count_xmas_vertical(input: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..input.len() {
        if let Some(n) = input.get(i) {
            for j in 0..n.len() {
                if let Some(c) = n.get(j) {
                    if c.clone() == 'X' {
                        // Up
                        if i >= 3 {
                            if let Some(m) = get_char_at_coordinates(&input, i-1, j) {
                                if m == 'M' {
                                    if let Some(a) = get_char_at_coordinates(&input, i-2, j) {
                                        if a == 'A' {
                                            if let Some(s) = get_char_at_coordinates(&input, i-3, j) {
                                                if s == 'S' {
                                                    count += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Down
                        if let Some(m) = get_char_at_coordinates(&input, i+1, j) {
                            if m == 'M' {
                                if let Some(a) = get_char_at_coordinates(&input, i+2, j) {
                                    if a == 'A' {
                                        if let Some(s) = get_char_at_coordinates(&input, i+3, j) {
                                            if s == 'S' {
                                                count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

fn count_xmas_diagonal(input: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..input.len() {
        if let Some(n) = input.get(i) {
            for j in 0..n.len() {
                if let Some(c) = n.get(j) {
                    if c.clone() == 'X' {
                        // Up + Right
                        if i >= 3 {
                            if let Some(m) = get_char_at_coordinates(&input, i-1, j+1) {
                                if m == 'M' {
                                    if let Some(a) = get_char_at_coordinates(&input, i-2, j+2) {
                                        if a == 'A' {
                                            if let Some(s) = get_char_at_coordinates(&input, i-3, j+3) {
                                                if s == 'S' {
                                                    count += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Up + Left
                        if i >= 3 && j >= 3 {
                            if let Some(m) = get_char_at_coordinates(&input, i-1, j-1) {
                                if m == 'M' {
                                    if let Some(a) = get_char_at_coordinates(&input, i-2, j-2) {
                                        if a == 'A' {
                                            if let Some(s) = get_char_at_coordinates(&input, i-3, j-3) {
                                                if s == 'S' {
                                                    count += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Down + Left
                        if j >= 3 {
                            if let Some(m) = get_char_at_coordinates(&input, i+1, j-1) {
                                if m == 'M' {
                                    if let Some(a) = get_char_at_coordinates(&input, i+2, j-2) {
                                        if a == 'A' {
                                            if let Some(s) = get_char_at_coordinates(&input, i+3, j-3) {
                                                if s == 'S' {
                                                    count += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Down + Right
                        if let Some(m) = get_char_at_coordinates(&input, i+1, j+1) {
                            if m == 'M' {
                                if let Some(a) = get_char_at_coordinates(&input, i+2, j+2) {
                                    if a == 'A' {
                                        if let Some(s) = get_char_at_coordinates(&input, i+3, j+3) {
                                            if s == 'S' {
                                                count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let (_, lines) = get_all_lines(&input).unwrap();
    let mut count: i32 = lines.iter().map(|line| count_xmas_horizontal(line.clone())).sum();
    count += count_xmas_vertical(lines.clone());
    count += count_xmas_diagonal(lines);

    println!("{count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_xmas_horizontal() {
        let tests = [
            (vec!['X', 'M', 'A', 'S'], 1),
            (vec!['S', 'A', 'M', 'X'], 1),
            (vec!['S', 'S', 'A', 'M', 'X'], 1),
            (vec!['X', 'X', 'M', 'A', 'S'], 1),
        ];
        tests.into_iter().for_each(|(input, output)| {
            let r = count_xmas_horizontal(input);
            assert_eq!(output, r);
        });
    }

    #[test]
    fn test_count_xmas_vertical() {
        let tests = [
            (vec![
             vec!['X', 'M', 'A', 'S'],
             vec!['M', 'M', 'A', 'A'],
             vec!['A', 'M', 'A', 'M'],
             vec!['S', 'M', 'A', 'X'],
            ], 2),
            (vec![
             vec!['X', 'M', 'A', 'S'],
             vec!['M', 'S', 'X', 'A'],
             vec!['A', 'A', 'M', 'M'],
             vec!['S', 'M', 'A', 'X'],
             vec!['S', 'X', 'S', 'X'],
            ], 4),
        ];
        tests.into_iter().for_each(|(input, output)| {
            let r = count_xmas_vertical(input);
            assert_eq!(output, r);
        });
    }

    #[test]
    fn test_count_xmas_diagonal() {
        let tests = [
            (vec![
             vec!['X', 'M', 'A', 'S'],
             vec!['M', 'M', 'A', 'A'],
             vec!['A', 'M', 'A', 'M'],
             vec!['X', 'M', 'A', 'S'],
            ], 2),
            (vec![
             vec!['S', 'M', 'A', 'X'],
             vec!['M', 'A', 'M', 'A'],
             vec!['A', 'A', 'M', 'M'],
             vec!['S', 'M', 'A', 'X'],
            ], 2),
        ];
        tests.into_iter().for_each(|(input, output)| {
            let r = count_xmas_diagonal(input);
            assert_eq!(output, r);
        });
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = read_to_string("example.txt")?;
        let (_, lines) = get_all_lines(&input).unwrap();
        let mut count: i32 = lines.iter().map(|line| count_xmas_horizontal(line.clone())).sum();
        count += count_xmas_vertical(lines.clone());
        count += count_xmas_diagonal(lines);

        assert_eq!(count, 18);
        Ok(())
    }
}
