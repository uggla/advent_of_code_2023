use std::collections::HashMap;
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

fn run(mut input: Vec<String>) -> u32 {
    let numbers = Vec::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let numbers: HashMap<&str, &str> = numbers.into_iter().collect();

    input
        .iter_mut()
        .map(|o| {
            let mut matches = Vec::new();
            for key in numbers.keys() {
                for i in o.match_indices(key) {
                    matches.push(i);
                }
            }
            matches.sort();
            dbg!(&matches);
            let mut matches: VecDeque<(usize, &str)> = matches.into();
            match matches.len() {
                0 => {}
                1 => {
                    let matches = matches.pop_front().unwrap().1;
                    *o = o.replace(matches, numbers[matches]);
                }
                _ => {
                    let (first_matches_index, first_matches) = matches.pop_front().unwrap();
                    let (last_matches_index, last_matches) = matches.pop_back().unwrap();

                    if first_matches_index + first_matches.len() <= last_matches_index {
                        // no overlap we can replace safely
                        *o = o
                            .replace(first_matches, numbers[first_matches])
                            .replace(last_matches, numbers[last_matches]);
                    } else {
                        // overlap
                        let overlap_size =
                            (first_matches_index + first_matches.len()) - last_matches_index;
                        let truncate_index = first_matches.len() - overlap_size;
                        let new_pattern =
                            format!("{}{}", &first_matches[..truncate_index], last_matches);
                        *o = o.replace(
                            &new_pattern,
                            &format!("{}{}", numbers[first_matches], numbers[last_matches]),
                        );
                        dbg!(overlap_size);
                        dbg!(&new_pattern);
                    }
                }
            }
            o
        })
        .map(|o| {
            dbg!(&o);
            o.chars()
                .filter_map(|o| o.to_digit(10))
                .collect::<VecDeque<u32>>()
        })
        .map(|mut o| {
            dbg!(&o);
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
    fn test_overlap() {
        let input = parse_input(Some(indoc!(
            "
            oneight
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 18);
    }

    #[test]
    fn test_overlap2() {
        // Overlap possibilities
        // oneight, twone, threeight, fiveight, sevenine, eightwo, eighthree, nineight
        let input = parse_input(Some(indoc!(
            "
            oneight
            twone
            threeight
            fiveight
            sevenine
            eightwo
            eighthree
            nineight
            15qhpvsevensixoneightt
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 18 + 21 + 38 + 58 + 79 + 82 + 83 + 98 + 18);
    }

    #[test]
    fn test_no_overlap() {
        let input = parse_input(Some(indoc!(
            "
            oneeight
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 18);
    }

    #[test]
    fn test_run() {
        let input = parse_input(Some(indoc!(
            "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 281);
    }
}
