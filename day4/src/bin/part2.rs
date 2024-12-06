use anyhow::Result;
use std::fs::read_to_string;
use nom::{
    character::complete::{anychar, char, i32, line_ending, one_of, space0}, combinator::opt, multi::many1, sequence::{preceded, terminated}, IResult
};

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

fn count_xmas(input: Vec<Vec<char>>) -> i32 {
    let mut count = 0;
    for i in 0..input.len() {
        if let Some(n) = input.get(i) {
            for j in 0..n.len() {
                if let Some(c) = n.get(j) {
                    if c.clone() == 'A' {
                        if i >= 1 && j >= 1 {
                            if let (Some(ul), Some(ur), Some(bl), Some(br)) = (
                                get_char_at_coordinates(&input, i-1, j-1), 
                                get_char_at_coordinates(&input, i-1, j+1), 
                                get_char_at_coordinates(&input, i+1, j-1), 
                                get_char_at_coordinates(&input, i+1, j+1)) {
                                if ((ul == 'M' && ur == 'M') && (br == 'S' && bl == 'S')) ||
                                   ((ul == 'S' && ur == 'S') && (br == 'M' && bl == 'M')) ||
                                   ((ul == 'S' && bl == 'S') && (br == 'M' && ur == 'M')) ||
                                   ((ul == 'M' && bl == 'M') && (br == 'S' && ur == 'S')) {
                                    count += 1;
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
    let count = count_xmas(lines);

    println!("{count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xmas() {
        let tests = [
            (vec![
             vec!['M', 'M', 'S'],
             vec!['M', 'A', 'A'],
             vec!['M', 'M', 'S'],
            ], 1),
            (vec![
             vec!['M', 'S', 'M'],
             vec!['A', 'A', 'M'],
             vec!['S', 'M', 'S'],
            ], 1),
            (vec![
             vec!['S', 'S', 'M'],
             vec!['A', 'A', 'M'],
             vec!['S', 'M', 'M'],
            ], 1),
            (vec![
             vec!['S', 'S', 'S'],
             vec!['A', 'A', 'M'],
             vec!['M', 'M', 'M'],
            ], 1),
            (vec![
             vec!['S', 'S', 'M'],
             vec!['A', 'A', 'M'],
             vec!['M', 'M', 'M'],
            ], 0),
            (vec![
             vec!['S', 'S', 'S'],
             vec!['A', 'A', 'M'],
             vec!['M', 'M', 'S'],
            ], 0),
            (vec![
             vec!['S', 'S', 'S'],
             vec!['A', 'A', 'M'],
             vec!['S', 'M', 'M'],
            ], 0),
            (vec![
             vec!['M', 'S', 'S'],
             vec!['A', 'A', 'M'],
             vec!['M', 'M', 'M'],
            ], 0),
        ];
        tests.into_iter().for_each(|(input, output)| {
            let r = count_xmas(input);
            assert_eq!(output, r);
        });
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = read_to_string("example.txt")?;
        let (_, lines) = get_all_lines(&input).unwrap();
        let count = count_xmas(lines);
        assert_eq!(count, 9);
        Ok(())
    }
}
