use nom::{
    character::complete::{line_ending, none_of},
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
    let (input, patterns) = separated_list1(line_ending, parse_pattern)(input)?;

    let data = Data { patterns };

    Ok((input, data))
}

fn parse_pattern(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, pattern) = many1(parse_line)(input)?;
    Ok((input, pattern))
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    let (input, characters) = many1(none_of("\n"))(input)?;
    let (input, _) = line_ending(input)?;
    Ok((input, characters))
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    patterns: Vec<Vec<Vec<char>>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SymAxis {
    X,
    Y,
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let sym_y = data
        .patterns
        .iter()
        .map(|p| find_symetry_axis(p, SymAxis::Y))
        .collect::<Vec<_>>();

    let sym_x = data
        .patterns
        .iter()
        .map(|p| {
            let pattern_transposed = (0..p[0].len())
                .map(|x| p.iter().map(|y| y[x]).collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            find_symetry_axis(&pattern_transposed, SymAxis::X)
        })
        .collect::<Vec<_>>();

    dbg!(&sym_x);
    dbg!(&sym_y);

    let output = data
        .patterns
        .iter()
        .enumerate()
        .flat_map(|(i, _p)| {
            let mut merged_axis = sym_y[i].clone();
            merged_axis.append(&mut sym_x[i].clone());
            merged_axis
        })
        .map(|(axis, i)| match axis {
            SymAxis::X => i + 1,
            SymAxis::Y => (i + 1) * 100,
        })
        .collect::<Vec<_>>();

    output.iter().sum::<usize>()
}

fn find_symetry_axis(p: &Vec<Vec<char>>, axis: SymAxis) -> Vec<(SymAxis, usize)> {
    p.windows(2)
        .enumerate()
        .filter_map(|y| {
            if y.1[0] == y.1[1] {
                let before = (0..=y.0)
                    .rev()
                    .flat_map(|i| p[i].to_vec())
                    .collect::<Vec<_>>();
                let after = ((y.0 + 1)..p.len())
                    .flat_map(|i| p[i].to_vec())
                    .collect::<Vec<_>>();

                let length = match before.len() < after.len() {
                    true => before.len(),
                    false => after.len(),
                };

                if before[..length] == after[..length] {
                    Some((axis, y.0))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
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
    fn test_run1() {
        let input = read_input(Some(indoc!(
            "
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
            " //
              // ###...###..
              // ###...###..
              // #.#.#.#...#
              // ...#..##.#.
              // ...#..#....
              // .###.#.###.
              // #..##.###..
              // ..#..#.#.##
              // ..#..#.#.##
              // #..##.###..
              // .###.#.###.
              // ...#..#....
              // ...#..##.#.
              // #.#.#.#.#.#
              // ###...###..
        )));

        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 405);
    }
}
