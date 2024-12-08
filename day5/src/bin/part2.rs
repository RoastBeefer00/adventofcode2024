use anyhow::Result;
use std::fs::read_to_string;
use nom::{
    bytes::complete::tag, character::complete::{i32, line_ending, space0}, combinator::opt, multi::many1, sequence::{preceded, terminated, tuple}, IResult
};

fn parse_rule(input: &str) -> IResult<&str, (i32, i32)> {
    terminated(
        tuple((i32, preceded(tag("|"), i32))),
        line_ending
    )(input)
}

fn parse_page(input: &str) -> IResult<&str, Vec<i32>> {
    terminated(
        many1(preceded(opt(tag(",")), i32)),
        line_ending
    )(input)
}

fn multiple_lines(input: &str) -> IResult<&str, (Vec<(i32, i32)>, Vec<Vec<i32>>)> {
    tuple((
        terminated(many1(parse_rule), line_ending), 
        many1(parse_page)
    ))(input)
}

#[derive(Debug)]
struct Pages {
    pages: Vec<Page>,
}

#[derive(Debug)]
struct Page {
    id: i32,
    before: Vec<i32>,
}

impl Pages {
    fn find_page(&mut self, page: i32) -> Option<&mut Page> {
        for p in self.pages.iter_mut() {
            if p.id == page {
                return Some(p)
            }
        }

        None
    }

    fn add(&mut self, page: i32, before: i32) {
        if let Some(p) = self.find_page(page) {
            p.before.push(before);
        } else {
            self.pages.push(Page::new(page, Some(vec![before])));
        }
    }
}

impl Page {
    fn new(id: i32, before: Option<Vec<i32>>) -> Self {
        if let Some(b) = before {
            Page {
                id,
                before: b
            }
        } else {
            Page {
                id,
                before: vec![]
            }
        }
    }

    fn can_be_before(&self, id: i32) -> bool {
        self.before.contains(&id)
    }
}

fn get_all_pages(pages: Vec<(i32, i32)>) -> Pages {
    let mut r = Pages {
        pages: Vec::new()
    };

    for (id, before) in pages.into_iter() {
        r.add(id, before);
    }

    r
}

fn is_right_order(pages: &mut Pages, sequence: &&mut Vec<i32>) -> bool {
    for (i, num) in sequence.iter().enumerate() {
        if i != sequence.len() - 1 {
            for j in i+1..sequence.len() {
                let s = sequence.get(j).unwrap();
                if let Some(p) = pages.find_page(s.clone()) {
                    if p.before.contains(&num) {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn fix_order(pages: &mut Pages, sequence: &mut Vec<i32>) {
    let len = sequence.len();
    let mut i = 0;
    let mut reset = false;
    while i < len {
        reset = false;
        let seq_clone = sequence.clone();
        let num = seq_clone.get(i).unwrap();
        if i != len - 1 {
            println!("{}", i);
            let mut j = i + 1;
            while j < len {
                let s = sequence.get(j).unwrap();
                if let Some(p) = pages.find_page(s.clone()) {
                    if p.before.contains(&num) {
                        sequence.swap(i, j);
                        i = 0;
                        reset = true;
                        j = len;
                    }
                }
                j += 1;
            }
        }
        if !reset {
            i += 1;
        }
    }

}

fn get_middle_item(v: Vec<i32>) -> i32 {
    v.get(v.len() / 2).unwrap().to_owned()
}

fn main() -> Result<()> {
    let input = read_to_string("input.txt")?;
    let (_, (rules, mut pages)) = multiple_lines(&input).unwrap();
    let mut ps = get_all_pages(rules);
    let mut invalid_pages = pages.iter_mut().filter(|s| !is_right_order(&mut ps, s)).collect::<Vec<_>>();
    invalid_pages.iter_mut().for_each(|s| fix_order(&mut ps, s));

    let answer: i32 = invalid_pages.into_iter().map(|p| get_middle_item(p.clone())).sum();
    println!("{answer}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse_line() {
    //     let tests = [
    //         ("1 2 3 4 5", vec![1, 2, 3, 4, 5]),
    //         ("5 2 3 4 1", vec![5, 2, 3, 4, 1]),
    //         ("1 22 333 4444 55555", vec![1, 22, 333, 4444, 55555]),
    //     ];
    //     tests.into_iter().for_each(|(input, output)| {
    //         let (_, r) = parse_line(input).unwrap();
    //         assert_eq!(output, r);
    //     });
    // }
    //
    // #[test]
    // fn test_safe() {
    //     let tests = [
    //         (vec![1, 2, 3, 4, 5], true),
    //         (vec![1, 2, 3, 4, 8], false),
    //         (vec![5, 4, 3, 2, 1], true),
    //         (vec![8, 4, 3, 2, 1], false),
    //         (vec![5, 2, 3, 4, 1], false),
    //         (vec![1, 22, 333, 4444, 55555], false),
    //     ];
    //     tests.into_iter().for_each(|(input, output)| {
    //         let r = is_safe(input);
    //         assert_eq!(output, r);
    //     });
    // }

    #[test]
    fn test_example() -> Result<()> {
        let input = read_to_string("example.txt")?;
        let (_, (rules, mut pages)) = multiple_lines(&input).unwrap();
        let mut ps = get_all_pages(rules);
        let mut invalid_pages = pages.iter_mut().filter(|s| !is_right_order(&mut ps, s)).collect::<Vec<_>>();
        println!("{:?}", invalid_pages);
        invalid_pages.iter_mut().for_each(|s| fix_order(&mut ps, s));

        println!("{:?}", invalid_pages);
        let mut one = vec![97, 75, 47, 61, 53];
        let mut two = vec![61, 29, 13];
        let mut three = vec![97, 75, 47, 29, 13];
        let corrected_pages: Vec<&mut Vec<i32>> = vec![
            &mut one,
            &mut two,
            &mut three
        ];
        assert_eq!(corrected_pages, invalid_pages);
        let answer: i32 = invalid_pages.into_iter().map(|p| get_middle_item(p.clone())).sum();

        assert_eq!(answer, 123);
        Ok(())
    }
}
