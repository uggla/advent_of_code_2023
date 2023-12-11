use nom::{
    character::complete::{line_ending, space1},
    multi::separated_list1,
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
    let (input, histories) = separated_list1(line_ending, parse_line)(input)?;

    let data = Data { histories };

    Ok((input, data))
}

fn parse_line(input: &str) -> IResult<&str, Vec<isize>> {
    let (input, history) = separated_list1(space1, nom::character::complete::i64)(input)?;
    let history = history
        .into_iter()
        .map(|x| x as isize)
        .collect::<Vec<isize>>();
    Ok((input, history))
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    histories: Vec<Vec<isize>>,
}

fn run(input: String) -> isize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let sequencies = data
        .histories
        .into_iter()
        .map(|h| recurse(h, Vec::new()))
        .collect::<Vec<Vec<Vec<isize>>>>();

    dbg!(&sequencies);

    let predictions = sequencies
        .iter()
        .map(|s| {
            s.iter()
                .rev()
                .map(|i| i.iter().next().unwrap())
                .fold(0, |acc, x| x - acc)
        })
        .sum::<isize>();

    dbg!(predictions)
}

fn recurse(history: Vec<isize>, mut history_saved: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    history_saved.push(history.clone());

    if history.iter().all(|x| *x == 0) {
        history_saved
    } else {
        let diff_each_step = history
            .windows(2)
            .map(|x| x[1] - x[0])
            .collect::<Vec<isize>>();
        recurse(diff_each_step, history_saved)
    }
}

fn main() {
    let input = read_input(None);

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
        let input = read_input(Some(indoc!(
            "
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 2);
    }
}
