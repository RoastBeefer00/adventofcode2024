use anyhow::Result;
use std::fs::read_to_string;
use nom::{
    bytes::complete::tag, character::complete::{anychar, i32}, multi::{many1, many_till}, sequence::{delimited, preceded, tuple}, IResult
};

fn parse_line(input: &str) -> IResult<&str, Vec<(Vec<char>, (i32, i32))>> {
    many1(
        many_till(
            anychar,
            delimited(
                tag("mul("),
                tuple((i32, preceded(tag(","), i32))),
                tag(")")
            )
        )
    )(input)
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let (_, r) = parse_line(&input).unwrap();
    let nums = r.into_iter().map(|(_, tup)| tup).collect::<Vec<(i32, i32)>>();
    let answer: i32 = nums.into_iter().map(|(num_one, num_two)| num_one * num_two).sum();
    println!("{answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let tests = [
            ("mul(1,2)", vec![(1, 2)]),
            ("icaestnhmul(1,2)", vec![(1, 2)]),
        ];
        tests.into_iter().for_each(|(input, output)| {
            let (_, r) = parse_line(input).unwrap();
            let nums = r.into_iter().map(|(_, tup)| tup).collect::<Vec<(i32, i32)>>();
            assert_eq!(output, nums);
        });
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = read_to_string("example.txt")?;
        let (_, r) = parse_line(&input).unwrap();
        let nums = r.into_iter().map(|(_, tup)| tup).collect::<Vec<(i32, i32)>>();
        let answer: i32 = nums.into_iter().map(|(num_one, num_two)| num_one * num_two).sum();
        assert_eq!(answer, 161);
        Ok(())
    }
}
