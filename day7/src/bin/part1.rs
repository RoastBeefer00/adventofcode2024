use anyhow::{Result, anyhow};
use std::fs::read_to_string;
use nom::{
    bytes::complete::tag, character::complete::{anychar, char, i32, line_ending, one_of, space0}, combinator::opt, multi::many1, sequence::{preceded, terminated, tuple}, IResult
};
use itertools::Itertools;

fn process_line(input: &str) -> IResult<&str, (i32, Vec<i32>)> {
    tuple((
        terminated(i32, tag(":")), 
        many1(
            preceded(
                tag(" "), 
                i32
            )
        )
    ))(input)
}

fn get_all_lines(input: &str) -> IResult<&str, Vec<(i32, Vec<i32>)>> {
    many1(terminated(process_line, line_ending))(input)
}

fn is_valid_combination(result: &i32, nums: &Vec<i32>, combination: Vec<char>) -> bool {
    assert_eq!(nums.len() - 1, combination.len());
    let mut total = 0;
    combination.
        iter()
        .enumerate()
        .for_each(|(i, op)| {
            if i == 0 {
                let first = nums.get(i).unwrap();
                let second = nums.get(i + 1).unwrap();
                match op {
                    '+' => {
                        let add = first + second;
                        total += add;
                    },
                    '*' => {
                        let add = first * second;
                        total += add;
                    },
                    _ => {},
                }
            } else {
                match op {
                    '+' => {
                        total += nums.get(i + 1).unwrap();
                    },
                    '*' => {
                        total *= nums.get(i + 1).unwrap();
                    },
                    _ => {},
                }
            }
        });

    // println!("{:?}\nResult: {}\nTotal: {}\nNums: {:?}\nCombination: {:?}\n\n", &total == result,result, total, nums, combination);
    &total == result
}

fn main() -> Result<()> {

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = read_to_string("example.txt")?;
        let (_, lines) = get_all_lines(&input).unwrap();
        let sum: i32 = lines
            .into_iter()
            .filter(|(result, nums)| {
                let mut operators = Vec::new();
                for _ in 0..nums.len() - 1 {
                    operators.push('+');
                    operators.push('*');
                }
                let combinations = operators.into_iter().combinations(nums.len() - 1).collect::<Vec<_>>();
                let count = combinations.into_iter().filter(|comb| {
                    is_valid_combination(&result, &nums, comb.clone())
                }).count();

                count > 0
            })
            .map(|(result, _)| result)
            .sum();
        assert_eq!(sum, 3749);
        Ok(())
    }
}

