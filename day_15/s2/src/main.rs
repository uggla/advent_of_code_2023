use std::collections::BTreeMap;

use nom::{
    bytes::complete::tag,
    multi::{many1, separated_list1},
    *,
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Data> {
    let (input, steps) = separated_list1(tag(","), parse_step)(input)?;

    let data = Data { steps };

    Ok((input, data))
}

fn parse_step(input: &str) -> IResult<&str, String> {
    let (input, step) = many1(character::complete::none_of(","))(input)?;
    let step: String = step.into_iter().collect();
    let step = step.trim().to_string();
    Ok((input, step))
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    steps: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Operation {
    Add,
    Remove,
}

impl From<char> for Operation {
    fn from(c: char) -> Self {
        match c {
            '=' => Operation::Add,
            '-' => Operation::Remove,
            _ => panic!("Unknown operation"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Lens {
    label: String,
    operation: Operation,
    focal_length: usize,
}

impl Lens {
    fn new(label: String, operation: Operation, focal_length: usize) -> Self {
        Self {
            label,
            operation,
            focal_length,
        }
    }

    fn hash(&self) -> usize {
        hash(&self.label)
    }
}

#[derive(Debug)]
struct LensBox(BTreeMap<usize, Vec<Lens>>);

impl LensBox {
    fn new() -> Self {
        let mut lensbox = BTreeMap::new();
        for i in 0..256 {
            lensbox.insert(i, vec![]);
        }
        Self(lensbox)
    }

    fn insert_lens(&mut self, boxnb: usize, lens: Lens) {
        match self
            .0
            .get(&boxnb)
            .unwrap()
            .iter()
            .position(|l| l.label == lens.label)
        {
            None => {
                self.0.get_mut(&boxnb).unwrap().push(lens);
            }
            Some(pos) => {
                *self.0.get_mut(&boxnb).unwrap().get_mut(pos).unwrap() = lens;
            }
        }
    }
    fn remove_lens(&mut self, boxnb: usize, lens: Lens) {
        match self
            .0
            .get(&boxnb)
            .unwrap()
            .iter()
            .position(|l| l.label == lens.label)
        {
            None => {}
            Some(pos) => {
                self.0.get_mut(&boxnb).unwrap().remove(pos);
            }
        }
    }

    fn focusing_values(&self) -> Vec<usize> {
        self.0
            .iter()
            .flat_map(|(ibox, lens_list)| {
                lens_list
                    .iter()
                    .enumerate()
                    .map(|(ilens, lens)| (ibox + 1) * (ilens + 1) * lens.focal_length)
                    .collect::<Vec<usize>>()
            })
            .collect()
    }

    fn focusing_power(&self) -> usize {
        self.focusing_values().iter().sum()
    }
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let lens_list = data
        .steps
        .iter()
        .map(|s| match s.contains('=') {
            true => {
                let v = s.split('=').collect::<Vec<&str>>();
                Lens::new(
                    v[0].to_string(),
                    Operation::from('='),
                    v[1].parse().unwrap(),
                )
            }
            false => {
                let v = s.split('-').collect::<Vec<&str>>();
                Lens::new(v[0].to_string(), Operation::from('-'), 0)
            }
        })
        .collect::<Vec<_>>();

    let mut lens_box = LensBox::new();

    for lens in lens_list.into_iter() {
        match lens.operation {
            Operation::Add => {
                lens_box.insert_lens(lens.hash(), lens);
            }
            Operation::Remove => {
                lens_box.remove_lens(lens.hash(), lens);
            }
        }
    }

    dbg!(lens_box.focusing_power())
}

fn main() {
    let input = read_input(None);

    let answer = run(input);

    println!("Answer: {}", answer);
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |mut acc, c| {
        acc = ((acc + c as usize) * 17) % 256;
        acc
    })
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
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_run1() {
        let input = read_input(Some(indoc!(
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 145);
    }
}
