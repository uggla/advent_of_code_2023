use std::ops::Add;

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, hex_digit1, line_ending, multispace1},
    multi::many1,
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
    let (input, cubes) = many1(parse_line)(input)?;

    let data = Data { cubes };

    Ok((input, data))
}

fn parse_line(input: &str) -> IResult<&str, Cube> {
    let (input, direction) = anychar(input)?;
    let (input, _) = multispace1(input)?;
    let (input, distance) = nom::character::complete::u32(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("(#")(input)?;
    let (input, color) = hex_digit1(input)?;
    let (input, _) = tag(")")(input)?;
    let (input, _) = line_ending(input)?;

    Ok((
        input,
        Cube {
            direction: Direction::new(direction),
            distance: distance as usize,
            color: color.to_string(),
        },
    ))
}

#[derive(Debug)]
struct Data {
    cubes: Vec<Cube>,
}

#[derive(Debug)]
struct Cube {
    direction: Direction,
    distance: usize,
    color: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, Ord, PartialOrd)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl From<usize> for Direction {
    fn from(x: usize) -> Self {
        match x {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!("Invalid direction"),
        }
    }
}

impl Direction {
    fn new(c: char) -> Self {
        match c {
            'R' => Self::Right,
            'D' => Self::Down,
            'L' => Self::Left,
            'U' => Self::Up,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Ord, PartialOrd, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn advance(&self, direction: Direction) -> Self {
        match direction {
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
        }
    }
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

fn run(input: String) -> usize {
    let (_, mut data) = parse(&input).unwrap();

    // Fix the data
    data.cubes = data
        .cubes
        .iter()
        .map(|cube| {
            let distance = usize::from_str_radix(&cube.color[..5], 16).unwrap();
            let direction = Direction::from(cube.color[5..].parse::<usize>().unwrap());

            Cube {
                direction,
                distance,
                color: cube.color.clone(),
            }
        })
        .collect();
    dbg!(&data);

    let mut grid: Vec<(Coord, String)> = Vec::new();

    let mut current_coord = Coord::new(0, 0);

    for cube in data.cubes {
        for _ in 0..cube.distance {
            current_coord = current_coord.advance(cube.direction);
            grid.push((current_coord, cube.color.clone()));
        }
    }

    // Offset the grid to have positive coordinates
    let min_x = grid.iter().map(|(c, _)| c.x).min().unwrap();
    let min_y = grid.iter().map(|(c, _)| c.y).min().unwrap();

    let grid = grid
        .into_iter()
        .map(|(k, _)| Coord::new(k.x + min_x.abs(), k.y + min_y.abs()))
        .collect::<Vec<Coord>>();

    // We cannot iterate the grid because there are to many positions
    // But we can calculate the polygone area with coordinates.
    // http://villemin.gerard.free.fr/GeomLAV/Polygone/Lacet.htm
    let area = polygon_area(&grid);
    // Found perimeter "experimentally" but the reason is here:
    // https://www.reddit.com/r/adventofcode/comments/18l8mao/2023_day_18_intuition_for_why_spoiler_alone/
    let perimeter = &grid.len() / 2 + 1;
    dbg!(area as usize + perimeter)
}

fn polygon_area(vertices: &[Coord]) -> isize {
    let mut area: isize = 0;
    let n = vertices.len();

    for i in 0..n {
        let p1 = &vertices[i];
        let p2 = &vertices[(i + 1) % n]; // To loop around to the first point
        area += p1.x * p2.y - p2.x * p1.y;
    }

    area.abs() / 2
}

#[allow(dead_code)]
fn is_inside(height: usize, width: usize, grid: &[(Coord, String)]) -> Vec<Coord> {
    let mut in_poly = Vec::new();
    let grid = grid.iter().map(|(c, _)| *c).collect::<Vec<Coord>>();
    for y in 0..height {
        for x in 0..width {
            let coord = Coord::new(x as isize, y as isize);
            if !grid.iter().any(|c| *c == coord) && inside(&coord, &grid) != 0 {
                in_poly.push(coord);
            }
        }
    }
    in_poly
}

// This function uses winding number algorithm to determine if a point is inside a polygon
#[allow(dead_code)]
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
#[allow(dead_code)]
fn is_left(seg_p0: &Coord, seg_p1: &Coord, point: &Coord) -> f64 {
    (seg_p1.x as f64 - seg_p0.x as f64) * (point.y as f64 - seg_p0.y as f64)
        - (point.x as f64 - seg_p0.x as f64) * (seg_p1.y as f64 - seg_p0.y as f64)
}

#[allow(dead_code)]
fn print_text_map(coordinates: &[(usize, usize, char)], width: usize, height: usize) {
    let mut grid = vec![vec!['.'; width]; height];

    // Place the points on the grid
    for &(x, y, v) in coordinates {
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
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 952408144115);
    }
}
