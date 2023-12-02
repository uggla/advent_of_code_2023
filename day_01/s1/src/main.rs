use std::collections::VecDeque;

fn parse_input(input: Option<&str>) -> Vec<String> {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };
    let output = input
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|o| o.to_string())
        .collect::<Vec<String>>();

    output
}

fn join_digits(first_elem: u32, last_elem: u32) -> u32 {
    format!("{}{}", first_elem, last_elem)
        .parse::<u32>()
        .unwrap()
}

fn run(input: Vec<String>) -> u32 {
    input
        .iter()
        .map(|o| {
            o.chars()
                .filter_map(|o| o.to_digit(10))
                .collect::<VecDeque<u32>>()
        })
        .map(|mut o| {
            if o.len() == 1 {
                let element = o.pop_front().unwrap();
                join_digits(element, element)
            } else {
                let first_elem = o.pop_front().unwrap();
                let last_elem = o.pop_back().unwrap();
                join_digits(first_elem, last_elem)
            }
        })
        .sum::<u32>()
}

fn main() {
    let input = parse_input(None);

    let answer = run(input);

    println!("Answer: {}", answer);
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use indoc::indoc;
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn test_fake() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 142);
    }
}
