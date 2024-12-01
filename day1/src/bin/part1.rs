use anyhow::Result;
use std::fs::read_to_string;
use nom::{
    character::complete::{i32, line_ending, space1}, combinator::opt, multi::many1, sequence::{preceded, terminated, tuple}, IResult
};

fn parse_line(input: &str) -> IResult<&str, (i32, i32)> {
    tuple((i32, preceded(space1, i32)))(input)
}

fn multiple_lines(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many1(terminated(parse_line, opt(line_ending)))(input)
}

fn main() -> Result<()> {
    let input = read_to_string("part1.txt")?;
    let (_, nums) = multiple_lines(&input).unwrap();
    let mut list_one = nums.iter().map(|(num_one, _)| num_one).collect::<Vec<_>>();
    let mut list_two = nums.iter().map(|(_, num_two)| num_two).collect::<Vec<_>>();
    list_one.sort();
    list_two.sort();
    assert_eq!(list_one.len(), list_two.len());
    let r: i32 = list_one.iter().zip(list_two.iter()).map(|(a, b)| {
        let value = *a - *b;
        value.abs()
    }).sum();
    println!("{r}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let tests = [
            ("123 456", (123, 456)),
            ("123  456", (123, 456)),
            ("1        2", (1, 2))
        ];
        tests.into_iter().for_each(|(input, output)| {
            let (_, r) = parse_line(input).unwrap();
            assert_eq!(output, r);
        });
    }

    #[test]
    fn test_multiple_lines() {
        let tests = [
            ("123 456", vec![(123, 456)]),
            ("123  456\n987 654", vec![(123, 456), (987, 654)])
        ];
        tests.into_iter().for_each(|(input, output)| {
            let (_, r) = multiple_lines(input).unwrap();
            assert_eq!(output, r);
        });
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = read_to_string("part1-example.txt")?;
        let (_, nums) = multiple_lines(&input).unwrap();
        let mut list_one = nums.iter().map(|(num_one, _)| num_one).collect::<Vec<_>>();
        let mut list_two = nums.iter().map(|(_, num_two)| num_two).collect::<Vec<_>>();
        list_one.sort();
        list_two.sort();
        assert_eq!(list_one.len(), list_two.len());
        let r: i32 = list_one.iter().zip(list_two.iter()).map(|(a, b)| {
            let value = *a - *b;
            value.abs()
        }).sum();
        assert_eq!(11, r);
        Ok(())
    }
}
