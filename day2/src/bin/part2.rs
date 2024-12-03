use anyhow::Result;
use std::fs::read_to_string;
use nom::{
    character::complete::{i32, line_ending, space0}, combinator::opt, multi::many1, sequence::{preceded, terminated}, IResult
};

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    many1(preceded(space0, i32))(input)
}

fn multiple_lines(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    many1(terminated(parse_line, opt(line_ending)))(input)
}

fn is_safe(report: Vec<i32>) -> bool {
    let mut is_safe = true;
    assert_eq!(report.len() > 2, true);
    let is_ascending = report[0] < report[1];
    report.iter().enumerate().for_each(|(i, num)| {
        if let Some(next) = report.get(i + 1) {
            if is_ascending && next <= num {
                is_safe = false;
            } else if !is_ascending && next >= num {
                is_safe = false;
            } else {
                let mut diff = num - next;
                diff = diff.abs();
                if diff > 3 {
                    is_safe = false;
                }
            }
        } 
    });

    is_safe
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let (_, reports) = multiple_lines(&input).unwrap();
    let count = reports.into_iter().filter(|report| {
        if !is_safe(report.clone()) {
            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                if is_safe(new_report) {
                    return true;
                }
            }
            false
        } else {
            true
        }
    }).count();
    println!("{count}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let tests = [
            ("1 2 3 4 5", vec![1, 2, 3, 4, 5]),
            ("5 2 3 4 1", vec![5, 2, 3, 4, 1]),
            ("1 22 333 4444 55555", vec![1, 22, 333, 4444, 55555]),
        ];
        tests.into_iter().for_each(|(input, output)| {
            let (_, r) = parse_line(input).unwrap();
            assert_eq!(output, r);
        });
    }

    #[test]
    fn test_safe() {
        let tests = [
            (vec![1, 2, 3, 4, 5], true),
            (vec![1, 2, 3, 4, 8], false),
            (vec![5, 4, 3, 2, 1], true),
            (vec![8, 4, 3, 2, 1], false),
            (vec![5, 2, 3, 4, 1], false),
            (vec![1, 22, 333, 4444, 55555], false),
        ];
        tests.into_iter().for_each(|(input, output)| {
            let r = is_safe(input);
            assert_eq!(output, r);
        });
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = read_to_string("example.txt")?;
        let (_, reports) = multiple_lines(&input).unwrap();
        let count = reports.into_iter().filter(|report| {
            if !is_safe(report.clone()) {
                for i in 0..report.len() {
                    let mut new_report = report.clone();
                    new_report.remove(i);
                    if is_safe(new_report) {
                        return true;
                    }
                }
                false
            } else {
                true
            }
        }).count();
        assert_eq!(count, 4);
        Ok(())
    }
}
