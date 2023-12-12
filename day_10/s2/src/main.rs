use core::panic;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Add,
};

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

    let data = pipes
        .iter()
        .enumerate()
        .flat_map(|y| {
            y.1.iter()
                .enumerate()
                .map(|x| (Coord::from((x.0 as isize, y.0 as isize)), *x.1))
                .collect::<Vec<(Coord, char)>>()
        })
        .collect::<BTreeMap<Coord, char>>();

    let data = Data {
        length: pipes[0].len(),
        grid: data,
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
    length: usize,
    grid: BTreeMap<Coord, char>,
}

impl Data {
    // Get all neighbours clockwise
    #[allow(dead_code)]
    fn get_neighbours(&self, coord: Coord) -> Vec<(Coord, char)> {
        let neighbours_coords: Vec<Coord> =
            Vec::from([(1, 0).into(), (0, 1).into(), (-1, 0).into(), (0, -1).into()]);

        let neighbours = neighbours_coords
            .iter()
            .filter_map(|nc| self.grid.get(&(coord + *nc)).map(|v| (coord + *nc, *v)))
            .collect::<Vec<(Coord, char)>>();
        neighbours
    }

    fn next_pos(&self, current_pos: Coord, prev_pos: Coord) -> (Coord, char) {
        println!("{:?}-{:?}", prev_pos, current_pos);
        let neighbours = self.get_neighbours_with_pipes(current_pos);
        dbg!(neighbours
            .into_iter()
            .filter(|n| n.0 != prev_pos)
            .nth(0)
            .unwrap())
    }

    // Get all possible neighbours clockwise depending on pipes
    fn get_neighbours_with_pipes(&self, coord: Coord) -> Vec<(Coord, char)> {
        let neighbours_coords: Vec<Coord> =
            Vec::from([(1, 0).into(), (0, 1).into(), (-1, 0).into(), (0, -1).into()]);

        let neighbours = neighbours_coords
            .iter()
            .filter_map(|nc| {
                //
                let current_coord_value = self.grid.get(&coord).unwrap();
                match self.grid.get(&(coord + *nc)) {
                    None => None,
                    Some(v) => {
                        let new_coord = coord + *nc;

                        match current_coord_value {
                            '-' => {
                                match nc {
                                    Coord { x: 1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: -1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            '7' => {
                                match nc {
                                    Coord { x: 0, y: 1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: -1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            //
                            '|' => {
                                match nc {
                                    Coord { x: 0, y: 1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: 0, y: -1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            'J' => {
                                match nc {
                                    Coord { x: -1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: 0, y: -1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            'L' => {
                                match nc {
                                    Coord { x: 1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: 0, y: -1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            'F' => {
                                match nc {
                                    Coord { x: 1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: 0, y: 1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => None,
                                }
                            }
                            'S' => {
                                match nc {
                                    Coord { x: 1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: 0, y: 1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'J' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: -1, y: 0 } => {
                                        //
                                        match v {
                                            '-' => Some((new_coord, *v)),
                                            'L' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    Coord { x: 0, y: -1 } => {
                                        //
                                        match v {
                                            '|' => Some((new_coord, *v)),
                                            'F' => Some((new_coord, *v)),
                                            '7' => Some((new_coord, *v)),
                                            'S' => Some((new_coord, *v)),
                                            _ => None,
                                        }
                                    }
                                    _ => panic!("Invalid coord"),
                                }
                            }
                            _ => panic!("Invalid value"),
                        }
                    }
                }
            })
            .collect::<Vec<(Coord, char)>>();
        neighbours
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

fn print_text_map(coordinates: &[(usize, usize)], width: usize, height: usize) {
    let mut grid = vec![vec!['.'; width]; height];

    // Place the points on the grid
    for &(x, y) in coordinates {
        if x < width && y < height {
            grid[y][x] = 'X'; // Assuming the origin (0,0) is at the top-left corner
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

// This function uses winding number algorithm to determine if a point is inside a polygon
#[allow(clippy::collapsible_else_if)]
fn inside(point: &Coord, polygon: &[Coord]) -> i32 {
    let mut wn = 0; // Winding number

    for i in polygon.windows(2) {
        let seg_p1 = &i[0];
        let seg_p2 = &i[1];

        if seg_p1.y <= point.y {
            if seg_p2.y > point.y && is_left(seg_p1, seg_p2, point) > 0.0 {
                wn += 1; // A upward crossing
            }
        } else {
            if seg_p2.y <= point.y && is_left(seg_p1, seg_p2, point) < 0.0 {
                wn -= 1; // A downward crossing
            }
        }
    }

    wn
}

// Helper function using a vectoriel product to determine if a point is to the left of a line segment
fn is_left(seg_p0: &Coord, seg_p1: &Coord, point: &Coord) -> f64 {
    (seg_p1.x as f64 - seg_p0.x as f64) * (point.y as f64 - seg_p0.y as f64)
        - (point.x as f64 - seg_p0.x as f64) * (seg_p1.y as f64 - seg_p0.y as f64)
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let start = data.grid.iter().find(|x| x.1 == &'S').unwrap();

    let mut current_pos = (*start.0, *start.1);
    let mut previous_pos = current_pos;
    let mut loop_edges = Vec::from([current_pos.0]);
    let mut loop_set = BTreeSet::new();
    let mut _iterations = 0;
    loop {
        let new_pos = data.next_pos(current_pos.0, previous_pos.0);
        previous_pos = current_pos;
        current_pos = new_pos;

        // Push loop coord into a set
        if current_pos.1 != '-' && current_pos.1 != '|' {
            loop_edges.push(current_pos.0);
        }
        loop_set.insert(current_pos.0);
        _iterations += 1;

        if current_pos.1 == 'S' {
            break;
        }
    }

    let tile_set = data
        .grid
        .iter()
        .map(|c| {
            //
            *c.0
        })
        .collect::<BTreeSet<_>>();

    let tile_not_in_loop = tile_set
        .symmetric_difference(&loop_set)
        .clone()
        .collect::<BTreeSet<_>>();

    let tile_inside_loop = tile_not_in_loop
        .iter()
        .filter(|c| inside(c, &loop_edges) != 0)
        .map(|c| **c)
        .collect::<Vec<_>>();

    print!("\n\n");
    println!("Tile not in loop:");
    print_text_map(
        &tile_not_in_loop
            .into_iter()
            .map(|c| (c.x as usize, c.y as usize))
            .collect::<Vec<(usize, usize)>>(),
        data.length,
        data.length,
    );
    print!("\n\n");
    println!("Loop tiles:");
    print_text_map(
        &loop_set
            .into_iter()
            .map(|c| (c.x as usize, c.y as usize))
            .collect::<Vec<(usize, usize)>>(),
        data.length,
        data.length,
    );
    print!("\n\n");
    println!("Loop edges:");
    print_text_map(
        &loop_edges
            .into_iter()
            .map(|c| (c.x as usize, c.y as usize))
            .collect::<Vec<(usize, usize)>>(),
        data.length,
        data.length,
    );
    print!("\n\n");
    println!("Tiles inside loop:");
    print_text_map(
        &tile_inside_loop
            .iter()
            .map(|c| (c.x as usize, c.y as usize))
            .collect::<Vec<(usize, usize)>>(),
        data.length,
        data.length,
    );

    dbg!(tile_inside_loop.len())
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
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_run2() {
        let input = read_input(Some(indoc!(
            "
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 8);
    }

    #[test]
    fn test_run3() {
        let input = read_input(Some(indoc!(
            "
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 10);
    }
}
