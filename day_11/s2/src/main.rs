use itertools::Itertools;
use std::ops::Add;

use nom::{
    bytes::complete::take_until, character::complete::line_ending, multi::many1,
    sequence::terminated, *,
};

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, Data> {
    let (input, pipes) = many1(parse_line)(input)?;

    let data = Data {
        length_x: pipes[0].len(),
        length_y: pipes.len(),
        grid: pipes,
    };

    Ok((input, data))
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    let (input, pipe) = terminated(take_until("\n"), line_ending)(input)?;
    let pipe = pipe.to_string().chars().collect();
    Ok((input, pipe))
}

#[derive(Debug, PartialEq, Eq)]
struct Data {
    length_x: usize,
    length_y: usize,
    grid: Vec<Vec<char>>,
}

impl Data {
    fn insert_row(&mut self, index: usize, new_row: Vec<char>) {
        if index <= self.grid.len() {
            self.grid.insert(index, new_row);
        } else {
            self.grid.push(new_row);
        }
        self.length_y += 1;
    }

    fn insert_column(&mut self, index: usize, new_value: char) {
        for row in self.grid.iter_mut() {
            if index <= row.len() {
                row.insert(index, new_value);
            } else {
                row.push(new_value);
            }
        }
        self.length_x += 1;
    }
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

fn distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    println!("{} {} {} {}", x1, y1, x2, y2);
    let dx = (x2 as isize - x1 as isize).abs();
    let dy = (y2 as isize - y1 as isize).abs();
    // let dist_square = (dx.pow(2) + dy.pow(2)) as f64;
    // dist_square.sqrt() as usize
    (dx + dy) as usize
}

fn run(input: String) -> usize {
    let (_, mut data) = parse(&input).unwrap();
    dbg!(&data);

    let insert_row_indices = data
        .grid
        .iter()
        .enumerate()
        .filter_map(|y| {
            if y.1.iter().all(|x| *x == '.') {
                Some(y.0)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    dbg!(&insert_row_indices);

    for (val, index) in insert_row_indices.into_iter().enumerate() {
        data.insert_row(val + index, vec!['.'; data.length_x]);
    }

    let insert_col_indices = (0..data.length_x)
        .filter(|x| {
            data.grid
                .iter()
                .map(|y| if y[*x] == '.' { Some('.') } else { None })
                .all(|x| x == Some('.'))
        })
        .collect::<Vec<usize>>();

    dbg!(&insert_col_indices);

    for (val, index) in insert_col_indices.into_iter().enumerate() {
        data.insert_column(val + index, '.');
    }

    println!("Map");
    print_text_map(
        &data
            .grid
            .iter()
            .enumerate()
            .flat_map(|(y, r)| {
                r.iter().enumerate().map(move |(x, v)| {
                    //
                    ((x, y), *v)
                })
            })
            .collect::<Vec<((usize, usize), char)>>(),
        data.length_x,
        data.length_y,
    );

    let mut galaxies = Vec::new();
    for y in data.grid.iter().enumerate() {
        for x in y.1.iter().enumerate() {
            if *x.1 == '#' {
                galaxies.push((x.0, y.0))
            }
        }
    }

    dbg!(&galaxies);

    let travels = galaxies.iter().combinations(2).collect::<Vec<_>>();

    dbg!(&travels.len());
    // ....#........
    // .........#...
    // #............
    // .............
    // .............
    // ........#....
    // .#...........
    // ............#
    // .............
    // .............
    // .........#...
    // #....#.......

    dbg!(distance(1, 6, 5, 11));
    dbg!(distance(4, 0, 9, 10));
    dbg!(distance(0, 2, 12, 7));
    dbg!(distance(0, 11, 5, 11));

    travels
        .iter()
        .map(|t| {
            //
            distance(t[0].0, t[0].1, t[1].0, t[1].1)
        })
        .sum::<usize>()

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
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
            "
        )));

        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 374);
    }
}
