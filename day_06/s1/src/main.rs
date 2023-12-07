use nom::{
    bytes::complete::tag, character::complete::multispace1, multi::separated_list1,
    sequence::delimited, *,
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

#[derive(Debug)]
struct Data {
    times: Vec<u32>,
    distances: Vec<u32>,
}

fn parse(input: &str) -> IResult<&str, Data> {
    let (input, _) = tag("Time: ")(input)?;
    let (input, times) = delimited(
        multispace1,
        separated_list1(multispace1, nom::character::complete::u32),
        tag("\n"),
    )(input)?;
    let (input, _) = tag("Distance: ")(input)?;
    let (input, distances) = delimited(
        multispace1,
        separated_list1(multispace1, nom::character::complete::u32),
        tag("\n"),
    )(input)?;

    Ok((input, Data { times, distances }))
}

const DIST_PER_MS: u32 = 1;

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    data.times
        .iter()
        .zip(data.distances.iter())
        .map(|(time, best_distance)| {
            (0..=*time)
                .map(move |press_time| {
                    // Calculate distance
                    let speed = press_time * DIST_PER_MS;
                    (time - press_time) * speed
                })
                .filter(|distance| *distance > *best_distance)
                .count()
        })
        .product::<usize>()
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
            Time:      7  15   30
            Distance:  9  40  200
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 288);
    }
}
