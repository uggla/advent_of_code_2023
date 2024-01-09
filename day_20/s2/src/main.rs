use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, alpha1, line_ending, space1},
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

fn parse(input: &str) -> IResult<&str, HashMap<String, Component>> {
    let (input, components) = many1(parse_line)(input)?;

    let components = components.into_iter().collect();

    Ok((input, components))
}
fn parse_line(input: &str) -> IResult<&str, (String, Component)> {
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
            Component::Broadcaster(Broadcaster::new(name.to_string(), dests))
        }
        "%" => Component::FlipFlop(FlipFlop::new(name.to_string(), dests)),
        "&" => {
            // conjonctions.push(name.to_string());
            Component::Conjunction(Conjunction::new(name.to_string(), dests))
        }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Connection {
    from: String,
    to: String,
}

impl Connection {
    fn new(from: String, to: String) -> Self {
        Self { from, to }
    }
}

impl From<(&str, &str)> for Connection {
    fn from((from, to): (&str, &str)) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
        }
    }
}

#[derive(Debug)]
enum Component {
    Broadcaster(Broadcaster),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

#[derive(Debug, Clone)]
struct FlipFlop {
    name: String,
    state: State,
    output_connection: Vec<Connection>,
}

impl FlipFlop {
    fn new(name: String, connections: Vec<Connection>) -> Self {
        Self {
            name,
            state: State::Off,
            output_connection: connections,
        }
    }

    #[allow(unused_variables)]
    fn high_pulse(
        &self,
        pulse_counter: &mut PulseCounter,
        outputs: &mut HashMap<String, Vec<(Connection, Pulse)>>,
    ) {
        // Nothing should happen here !
        // The outputs should not be touched in this case and especially not cleared!
    }

    fn low_pulse(
        &mut self,
        pulse_counter: &mut PulseCounter,
        outputs: &mut HashMap<String, Vec<(Connection, Pulse)>>,
    ) {
        if self.state == State::Off {
            self.state = State::On;
            let mut out = Vec::new();
            for conn in self.output_connection.iter() {
                pulse_counter.high += 1;
                println!("{}: sends a high pulse to {}", self.name, conn.to);
                out.push((conn.clone(), Pulse::High));
            }
            outputs.insert(self.name.clone(), out);
        } else {
            self.state = State::Off;
            let mut out = Vec::new();
            for conn in self.output_connection.iter() {
                pulse_counter.low += 1;
                println!("{}: sends a low pulse to {}", self.name, conn.to);
                out.push((conn.clone(), Pulse::Low));
            }
            outputs.insert(self.name.clone(), out);
        }
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    name: String,
    output_connection: Vec<Connection>,
    input_connections: Vec<Connection>,
    inputs: Vec<(Connection, Pulse)>,
}

impl Conjunction {
    fn new(name: String, connections: Vec<Connection>) -> Self {
        Self {
            name,
            output_connection: connections,
            input_connections: Vec::new(),
            inputs: Vec::new(),
        }
    }
    fn high_pulse(
        &mut self,
        pulse_counter: &mut PulseCounter,
        outputs: &mut HashMap<String, Vec<(Connection, Pulse)>>,
    ) {
        self.get_outputs(outputs, pulse_counter);
    }

    fn low_pulse(
        &mut self,
        pulse_counter: &mut PulseCounter,
        outputs: &mut HashMap<String, Vec<(Connection, Pulse)>>,
    ) {
        self.get_outputs(outputs, pulse_counter);
    }

    fn get_outputs(
        &mut self,
        outputs: &mut HashMap<String, Vec<(Connection, Pulse)>>,
        pulse_counter: &mut PulseCounter,
    ) {
        self.get_inputs(outputs);
        outputs.get_mut(&self.name).unwrap().clear();
        if self
            .inputs
            .iter()
            .map(|(_, p)| p)
            .all(|p| *p == Pulse::High)
        {
            for conn in self.output_connection.iter() {
                println!("{}: sends a low pulse to {}", self.name, conn.to);
                pulse_counter.low += 1;
                outputs
                    .get_mut(&self.name)
                    .unwrap()
                    .push((conn.clone(), Pulse::Low));
            }
        } else {
            for conn in self.output_connection.iter() {
                println!("{}: sends a high pulse to {}", self.name, conn.to);
                pulse_counter.high += 1;
                outputs
                    .get_mut(&self.name)
                    .unwrap()
                    .push((conn.clone(), Pulse::High));
            }
        }
    }

    fn get_inputs(&mut self, outputs: &HashMap<String, Vec<(Connection, Pulse)>>) {
        self.inputs.clear();
        for input in self.input_connections.iter() {
            if let Some(output) = outputs
                .get(&input.from)
                .unwrap()
                .iter()
                .find(|(conn, _p)| *conn == *input)
            {
                self.inputs.push(output.clone());
            } else {
                self.inputs.push((input.clone(), Pulse::Low));
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Broadcaster {
    name: String,
    output_connection: Vec<Connection>,
}
impl Broadcaster {
    fn new(name: String, connections: Vec<Connection>) -> Self {
        Self {
            name,
            output_connection: connections,
        }
    }
    fn high_pulse(
        &mut self,
        pulse_counter: &mut PulseCounter,
        outputs: &mut HashMap<String, Vec<(Connection, Pulse)>>,
    ) {
        outputs.get_mut(&self.name).unwrap().clear();
        for conn in self.output_connection.iter_mut() {
            println!("{}: sends a low pulse to {}", self.name, conn.to);
            pulse_counter.high += 1;
            outputs
                .get_mut(&self.name)
                .unwrap()
                .push((conn.clone(), Pulse::High));
        }
    }

    fn low_pulse(
        &mut self,
        pulse_counter: &mut PulseCounter,
        outputs: &mut HashMap<String, Vec<(Connection, Pulse)>>,
    ) {
        outputs.get_mut(&self.name).unwrap().clear();
        for conn in self.output_connection.iter() {
            println!("{}: sends a low pulse to {}", self.name, conn.to);
            pulse_counter.low += 1;
            outputs
                .get_mut(&self.name)
                .unwrap()
                .push((conn.clone(), Pulse::Low));
        }
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let tmp = a;
        a = b;
        b = tmp % b;
    }
    a
}

fn run(input: String) -> usize {
    let (_, mut components) = parse(&input).unwrap();

    let connections = components
        .iter()
        .flat_map(|(_name, component)| match component {
            Component::Broadcaster(broadcaster) => broadcaster.output_connection.clone(),
            Component::FlipFlop(flip_flop) => flip_flop.output_connection.clone(),
            Component::Conjunction(conjonction) => conjonction.output_connection.clone(),
        })
        .collect::<Vec<_>>();

    let mut outputs: HashMap<String, Vec<(Connection, Pulse)>> = HashMap::new();
    // Initialise all outputs
    for name in components.keys() {
        outputs.insert(name.clone(), Vec::new());
    }

    // Populate input connections for conjonction components
    for comp in components.iter_mut() {
        if let Component::Conjunction(conjonction) = comp.1 {
            for Connection { from, to } in connections.iter() {
                if to == &conjonction.name {
                    conjonction
                        .input_connections
                        .push(Connection::new(from.clone(), to.clone()));
                }
            }
        }
    }

    dbg!(&components);

    let mut pulse_counter = PulseCounter::new();
    let mut stack: VecDeque<(Connection, Pulse)> = VecDeque::new();

    let mut counter: usize = 0;
    let mut lcms = HashMap::new();

    'button_loop: loop {
        // Count the number of time the button is pressed
        counter += 1;

        stack.push_back((Connection::from(("button", "broadcaster")), Pulse::Low));
        println!("button: sends a low pulse to broadcaster");
        pulse_counter.low += 1;
        while !stack.is_empty() {
            let (conn, pulse) = stack.pop_front().unwrap();
            // rx low means mf(inv) high.
            // mf(inv) high means all inputs connected to mf are low.
            // &sh -> &mf -> rx
            // &mz -> &mf -> rx
            // &bh -> &mf -> rx
            // &jf -> &mf -> rx
            if conn.to == "sh" && pulse == Pulse::Low {
                lcms.insert("sh", counter);
            }
            if conn.to == "mz" && pulse == Pulse::Low {
                lcms.insert("mz", counter);
            }
            if conn.to == "bh" && pulse == Pulse::Low {
                lcms.insert("bh", counter);
            }
            if conn.to == "jf" && pulse == Pulse::Low {
                lcms.insert("jf", counter);
            }
            if lcms.contains_key("sh")
                && lcms.contains_key("mz")
                && lcms.contains_key("bh")
                && lcms.contains_key("jf")
            {
                break 'button_loop;
            }
            if conn.to == "rx" && pulse == Pulse::Low {
                break 'button_loop;
            }
            if let Some(component) = components.get_mut(&conn.to) {
                match component {
                    Component::Broadcaster(comp) => match pulse {
                        Pulse::Low => {
                            comp.low_pulse(&mut pulse_counter, &mut outputs);
                            for output in outputs.get(&comp.name).unwrap() {
                                stack.push_back(output.clone());
                            }
                        }
                        Pulse::High => {
                            comp.high_pulse(&mut pulse_counter, &mut outputs);
                            for output in outputs.get(&comp.name).unwrap() {
                                stack.push_back(output.clone());
                            }
                        }
                    },
                    Component::FlipFlop(comp) => match pulse {
                        Pulse::Low => {
                            comp.low_pulse(&mut pulse_counter, &mut outputs);
                            for output in outputs.get(&comp.name).unwrap() {
                                stack.push_back(output.clone());
                            }
                        }
                        Pulse::High => {
                            comp.high_pulse(&mut pulse_counter, &mut outputs);
                            // In this case nothing happens.
                            // The component produce no new outputs so nothing should be pushed on the stack
                        }
                    },
                    Component::Conjunction(comp) => match pulse {
                        Pulse::Low => {
                            comp.low_pulse(&mut pulse_counter, &mut outputs);
                            for output in outputs.get(&comp.name).unwrap() {
                                stack.push_back(output.clone());
                            }
                        }
                        Pulse::High => {
                            comp.high_pulse(&mut pulse_counter, &mut outputs);
                            for output in outputs.get(&comp.name).unwrap() {
                                stack.push_back(output.clone());
                            }
                        }
                    },
                }
            }
        }
    }
    dbg!(&pulse_counter);
    dbg!(&lcms);
    let bh = *lcms.get("bh").unwrap();
    let mz = *lcms.get("mz").unwrap();
    let jf = *lcms.get("jf").unwrap();
    let sh = *lcms.get("sh").unwrap();

    lcm(lcm(lcm(bh, mz), jf), sh)
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

    #[ignore]
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

    #[ignore]
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
