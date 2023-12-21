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

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let output: usize = data.steps.iter().map(|s| hash(s)).sum();
    dbg!(output)
}

fn main() {
    let input = read_input(None);

    let answer = run(input);

    println!("Answer: {}", answer);
}

fn hash(s: &str) -> usize {
    s.chars().fold(0, |mut acc, c| {
        acc = dbg!(((acc + c as usize) * 17) % 256);
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
        assert_eq!(answer, 1320);
    }
}
