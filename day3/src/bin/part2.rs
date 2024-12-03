use anyhow::Result;
use std::fs::read_to_string;
use nom::{
    bytes::complete::tag, character::complete::{anychar, i32}, combinator::opt, multi::{many0, many1, many_till}, sequence::{delimited, preceded, tuple}, IResult
};

fn parse_line(input: &str) -> IResult<&str, ((Vec<char>, (i32, i32)), Vec<(Vec<char>, (i32, i32))>)> {
    tuple((
        many_till(
            anychar,
            delimited(
                tag("mul("),
                tuple((i32, preceded(tag(","), i32))),
                tag(")")
            )
        ),
        many0(
                preceded(
                    many_till(
                        anychar,
                        tag("do()"),
                    ),
                    many_till(
                        anychar,
                        delimited(
                            tag("mul("),
                            tuple((i32, preceded(tag(","), i32))),
                            tag(")")
                        )
                    )
                )
        )
    ))(input)
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let (_, (first, rest)) = parse_line(&input).unwrap();
    let mut nums = rest.into_iter().map(|(_, tup)| tup).collect::<Vec<(i32, i32)>>();
    nums.insert(0, first.1);
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
            let (_, (first, rest)) = parse_line(&input).unwrap();
            let mut nums = rest.into_iter().map(|(_, tup)| tup).collect::<Vec<(i32, i32)>>();
            nums.insert(0, first.1);
            assert_eq!(output, nums);
        });
    }

    #[test]
    fn test_example() -> Result<()> {
        let input = read_to_string("example2.txt")?;
        let (_, (first, rest)) = parse_line(&input).unwrap();
        let mut nums = rest.into_iter().map(|(_, tup)| tup).collect::<Vec<(i32, i32)>>();
        nums.insert(0, first.1);
        let answer: i32 = nums.into_iter().map(|(num_one, num_two)| num_one * num_two).sum();
        assert_eq!(answer, 48);
        Ok(())
    }
}
