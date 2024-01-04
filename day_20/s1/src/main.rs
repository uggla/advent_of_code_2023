use std::collections::HashMap;
use std::fmt::Debug;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha0, alpha1, digit1, line_ending, space1},
    multi::{many1, separated_list0, separated_list1},
    *,
};

use nom::character::complete::char as nomchar;

fn read_input(input: Option<&str>) -> String {
    let input = match input {
        None => include_str!("../../input.txt"),
        Some(x) => x,
    };

    input.to_string()
}

fn parse(input: &str) -> IResult<&str, HashMap<String, Box<dyn Component>>> {
    let (input, components) = many1(parse_line)(input)?;

    let components = components
        .into_iter()
        .map(|(name, component)| (name, component))
        .collect();

    Ok((input, components))
}
fn parse_line(input: &str) -> IResult<&str, (String, Box<dyn Component>)> {
    let (input, component) = alt((tag("broadcaster"), tag("%"), tag("&")))(input)?; // % or &(input)?;
    let (input, mut name) = alpha0(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, dests) = alt((
        separated_list1(tag(","), parse_multi_dest),
        parse_unique_dest,
    ))(input)?;
    let (input, _) = line_ending(input)?;

    let dests = dests
        .into_iter()
        .map(|x| Connection::new(name.to_string(), x.trim().to_string()))
        .collect::<Vec<Connection>>();

    let component = match component {
        "broadcaster" => {
            name = "broadcaster";
            let dests = dests
                .into_iter()
                .map(|Connection { from: _, to }| Connection::new(name.to_string(), to))
                .collect();
            Box::new(Broadcaster::new(name.to_string(), dests)) as Box<dyn Component>
        }
        "%" => Box::new(FlipFlop::new(name.to_string(), dests)) as Box<dyn Component>,
        "&" => Box::new(Conjonction::new(name.to_string(), dests)) as Box<dyn Component>,
        _ => panic!("unknown component"),
    };

    Ok((input, (name.to_string(), component)))
}

fn parse_unique_dest(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, dest) = alpha1(input)?;

    Ok((input, vec![dest]))
}

fn parse_multi_dest(input: &str) -> IResult<&str, &str> {
    let (input, _) = space1(input)?;
    let (input, dest) = alpha1(input)?;

    Ok((input, dest))
}
//
// fn parse_rules(input: &str) -> IResult<&str, Rule> {
//     let (input, rule) = alt((parse_c_rule, parse_d_rule))(input)?;
//     Ok((input, rule))
// }
//
// fn parse_c_rule(input: &str) -> IResult<&str, Rule> {
//     let (input, rate) = alt((nomchar('x'), nomchar('m'), nomchar('a'), nomchar('s')))(input)?;
//     let (input, op) = alt((nomchar('<'), nomchar('>')))(input)?;
//     let (input, value) = digit1(input)?;
//     let (input, _) = tag(":")(input)?;
//     let (input, destination) = alpha1(input)?;
//     let rule = Rule {
//         rate: Some(rate),
//         op: Some(Op::from(op)),
//         value: Some(value.parse().unwrap()),
//         dest: destination.to_string(),
//     };
//     Ok((input, rule))
// }
//
// fn parse_d_rule(input: &str) -> IResult<&str, Rule> {
//     let (input, destination) = alpha1(input)?;
//     let rule = Rule {
//         rate: None,
//         op: None,
//         value: None,
//         dest: destination.to_string(),
//     };
//     Ok((input, rule))
// }
//
// #[derive(Debug)]
// struct Data {
//     workflow: HashMap<String, Vec<Rule>>,
//     parts: Vec<Part>,
// }

#[derive(Debug, Clone, Copy)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    On,
    Off,
}

#[derive(Debug)]
struct PulseCounter {
    low: usize,
    high: usize,
}

impl PulseCounter {
    fn new() -> Self {
        Self { low: 0, high: 0 }
    }
}

#[derive(Debug, Clone)]
struct Connection {
    from: String,
    to: String,
}

impl Connection {
    fn new(from: String, to: String) -> Self {
        Self { from, to }
    }
}

trait Component: Debug {
    fn high_pulse(&self, pulse_counter: &mut PulseCounter);
    fn low_pulse(&mut self, pulse_counter: &mut PulseCounter);
    fn get_outputs(&self) -> Vec<(Connection, Pulse)>;
}

#[derive(Debug, Clone)]
struct FlipFlop {
    name: String,
    state: State,
    output_connection: Vec<Connection>,
    outputs: Vec<(Connection, Pulse)>,
}

impl FlipFlop {
    fn new(name: String, connections: Vec<Connection>) -> Self {
        Self {
            name,
            state: State::Off,
            output_connection: connections,
            outputs: Vec::new(),
        }
    }
}

impl Component for FlipFlop {
    fn high_pulse(&self, pulse_counter: &mut PulseCounter) {
        println!("{}: receive a high pulse", self.name);
    }

    fn low_pulse(&mut self, pulse_counter: &mut PulseCounter) {
        println!("{}: receive a low pulse", self.name);
        if self.state == State::Off {
            self.state = State::On;
            pulse_counter.high += 1;
            for con in self.output_connection.iter() {
                self.outputs.push((con.clone(), Pulse::High));
            }
        } else {
            self.state = State::Off;
            pulse_counter.low += 1;
            for con in self.output_connection.iter() {
                self.outputs.push((con.clone(), Pulse::Low));
            }
        }
    }

    fn get_outputs(&self) -> Vec<(Connection, Pulse)> {
        self.outputs.clone()
    }
}

#[derive(Debug, Clone)]
struct Conjonction {
    name: String,
    state: State,
    output_connection: Vec<Connection>,
    input_connection: Vec<Connection>,
    outputs: Vec<(Connection, Pulse)>,
}

impl Conjonction {
    fn new(name: String, connections: Vec<Connection>) -> Self {
        Self {
            name,
            state: State::Off,
            output_connection: connections,
            input_connection: Vec::new(),
            outputs: Vec::new(),
        }
    }
}

impl Component for Conjonction {
    fn high_pulse(&self, pulse_counter: &mut PulseCounter) {
        println!("{}: receive a high pulse", self.name);
        // self.low_pulse(pulse_counter);
    }

    fn low_pulse(&mut self, pulse_counter: &mut PulseCounter) {
        println!("{}: receive a low pulse", self.name);
        todo!();
    }

    fn get_outputs(&self) -> Vec<(Connection, Pulse)> {
        self.outputs.clone()
    }
}

#[derive(Debug, Clone)]
struct Broadcaster {
    name: String,
    output_connection: Vec<Connection>,
    outputs: Vec<(Connection, Pulse)>,
}
impl Broadcaster {
    fn new(name: String, connections: Vec<Connection>) -> Self {
        Self {
            name,
            output_connection: connections,
            outputs: Vec::new(),
        }
    }
}

impl Component for Broadcaster {
    fn high_pulse(&self, pulse_counter: &mut PulseCounter) {
        println!("{}: receive a high pulse", self.name);
        // self.low_pulse(pulse_counter);
    }

    fn low_pulse(&mut self, pulse_counter: &mut PulseCounter) {
        todo!();
    }

    fn get_outputs(&self) -> Vec<(Connection, Pulse)> {
        self.outputs.clone()
    }
}
#[derive(Debug, Clone)]
struct Button {
    name: String,
    output_connection: Vec<Connection>,
    outputs: Vec<(Connection, Pulse)>,
}
impl Button {
    fn new(name: String, output: Vec<String>) -> Self {
        Self {
            name,
            output_connection: Vec::new(),
            outputs: Vec::new(),
        }
    }
}

impl Component for Button {
    fn high_pulse(&self, pulse_counter: &mut PulseCounter) {
        println!("{}: receive a high pulse", self.name);
        // self.low_pulse(pulse_counter);
    }

    fn low_pulse(&mut self, pulse_counter: &mut PulseCounter) {
        todo!();
    }

    fn get_outputs(&self) -> Vec<(Connection, Pulse)> {
        self.outputs.clone()
    }
}

fn run(input: String) -> usize {
    let (_, components) = parse(&input).unwrap();
    dbg!(&components);

    todo!();
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
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 32000000);
    }

    #[test]
    fn test_run2() {
        let input = read_input(Some(indoc!(
            "
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 11687500);
    }
}
