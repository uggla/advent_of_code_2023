use itertools::Itertools;
use std::ops::Add;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{anychar, line_ending, multispace1, none_of, not_line_ending},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::terminated,
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
    // let characters = characters.to_string().chars().collect();
    Ok((input, characters))
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    patterns: Vec<Vec<Vec<char>>>,
}

impl Data {
    // fn insert_row(&mut self, index: usize, new_row: Vec<char>) {
    //     if index <= self.grid.len() {
    //         self.grid.insert(index, new_row);
    //     } else {
    //         self.grid.push(new_row);
    //     }
    //     self.length_y += 1;
    // }
    //
    // fn insert_column(&mut self, index: usize, new_value: char) {
    //     for row in self.grid.iter_mut() {
    //         if index <= row.len() {
    //             row.insert(index, new_value);
    //         } else {
    //             row.push(new_value);
    //         }
    //     }
    //     self.length_x += 1;
    // }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl From<(isize, isize)> for Coord {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, rhs: Coord) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn print_text_map(coordinates: &[((usize, usize), char)], width: usize, height: usize) {
    let mut grid = vec![vec!['.'; width]; height];

    // Place the points on the grid
    for &((x, y), v) in coordinates {
        if x < width && y < height {
            grid[y][x] = v; // Assuming the origin (0,0) is at the top-left corner
        }
    }

    // Print the grid row by row
    for row in grid {
        for cell in row {
            print!("{}", cell);
        }
        println!(); // Newline at the end of each row
    }
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let sym_y = data
        .patterns
        .iter()
        .map(|p| {
            p.windows(2)
                .enumerate()
                .filter_map(|y| if y.1[0] == y.1[1] { Some(y.0) } else { None })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let sym_x = data
        .patterns
        .iter()
        .map(|p| {
            let columns = (0..p[0].len())
                .map(|x| p.iter().map(|y| y[x]).collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            columns
                .windows(2)
                .enumerate()
                .filter_map(|c| if c.1[0] == c.1[1] { Some(c.0) } else { None })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    dbg!(&sym_x);
    dbg!(&sym_y);

    let output = data
        .patterns
        .iter()
        .enumerate()
        .map(|p| {
            let lx = p.1[0].len();
            let ly = p.1.len();

            let xaxis = lx / 2;
            let yaxis = ly / 2;

            let bestx = sym_x[p.0]
                .iter()
                .map(|x| ((*x as isize - xaxis as isize).abs(), *x))
                .min()
                .unwrap_or((100, 0));

            let besty = sym_y[p.0]
                .iter()
                .map(|y| ((*y as isize - yaxis as isize).abs(), *y))
                .min()
                .unwrap_or((100, 0));

            dbg!(&bestx, &besty);

            if bestx.0 < besty.0 {
                bestx.1 + 1
            } else {
                (besty.1 + 1) * 100
            }
        })
        .collect::<Vec<_>>();

    dbg!(output.iter().sum::<usize>())

    // let insert_col_indices = (0..data.length_x)
    //     .filter(|x| {
    //         data.grid
    //             .iter()
    //             .map(|y| if y[*x] == '.' { Some('.') } else { None })
    //             .all(|x| x == Some('.'))
    //     })
    //     .collect::<Vec<usize>>();

    // println!("Map");
    // print_text_map(
    //     &data.patterns[1]
    //         .iter()
    //         .enumerate()
    //         .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, v)| ((x, y), *v)))
    //         .collect::<Vec<((usize, usize), char)>>(),
    //     9,
    //     7,
    // );
    //
    // todo!();
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

            ##..#.#..##
            ###...#....
            ##.##.##...
            ....###..##
            ####..###..
            ##.####....
            ...####.###
            ###...##...
            ##...#..###
            ##...#.....
            .##..##.###
            "
        )));

        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 405);
    }
}
