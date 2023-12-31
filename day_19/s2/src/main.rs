use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, digit1, line_ending},
    multi::{many1, separated_list1},
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

fn parse(input: &str) -> IResult<&str, Data> {
    let (input, workflow) = many1(parse_wf_line)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, parts) = many1(parse_p_line)(input)?;

    let workflow = workflow
        .iter()
        .cloned()
        .collect::<HashMap<String, Vec<Rule>>>();
    let data = Data { workflow, parts };
    Ok((input, data))
}
fn parse_p_line(input: &str) -> IResult<&str, Part> {
    let (input, _) = tag("{x=")(input)?;
    let (input, x) = digit1(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, m) = digit1(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, a) = digit1(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, s) = digit1(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = line_ending(input)?;
    Ok((
        input,
        Part {
            x: x.parse().unwrap(),
            m: m.parse().unwrap(),
            a: a.parse().unwrap(),
            s: s.parse().unwrap(),
        },
    ))
}

fn parse_wf_line(input: &str) -> IResult<&str, (String, Vec<Rule>)> {
    let (input, name) = take_until("{")(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, rules) = separated_list1(tag(","), parse_rules)(input)?;
    let (input, _) = tag("}")(input)?;
    let (input, _) = line_ending(input)?;

    Ok((input, (name.to_string(), rules)))
}

fn parse_rules(input: &str) -> IResult<&str, Rule> {
    let (input, rule) = alt((parse_c_rule, parse_d_rule))(input)?;
    Ok((input, rule))
}

fn parse_c_rule(input: &str) -> IResult<&str, Rule> {
    let (input, rate) = alt((nomchar('x'), nomchar('m'), nomchar('a'), nomchar('s')))(input)?;
    let (input, op) = alt((nomchar('<'), nomchar('>')))(input)?;
    let (input, value) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, destination) = alpha1(input)?;
    let rule = Rule {
        rate: Some(rate),
        op: Some(Op::from(op)),
        value: Some(value.parse().unwrap()),
        dest: destination.to_string(),
    };
    Ok((input, rule))
}

fn parse_d_rule(input: &str) -> IResult<&str, Rule> {
    let (input, destination) = alpha1(input)?;
    let rule = Rule {
        rate: None,
        op: None,
        value: None,
        dest: destination.to_string(),
    };
    Ok((input, rule))
}

#[derive(Debug)]
struct Data {
    workflow: HashMap<String, Vec<Rule>>,
    parts: Vec<Part>,
}

#[derive(Debug, Clone)]
struct Rule {
    rate: Option<char>,
    op: Option<Op>,
    value: Option<usize>,
    dest: String,
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Sup,
    Inf,
}

impl From<char> for Op {
    fn from(c: char) -> Self {
        match c {
            '<' => Op::Inf,
            '>' => Op::Sup,
            _ => panic!("Unknown op: {}", c),
        }
    }
}

#[derive(Debug, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let mut accepted = Vec::new();
    for part in data.parts.iter() {
        let mut wf_name = String::from("in");
        // Use a loop and do not recurse into the workflow to avoid stack overflow
        loop {
            // dbg!(&wf_name);
            match process_wf(&data.workflow, wf_name.clone(), part.clone()).unwrap() {
                "A" => {
                    accepted.push(part);
                    break;
                }
                "R" => {
                    break;
                }
                name => wf_name = name.to_string(),
            }
        }
    }
    dbg!(&accepted);
    accepted.iter().map(|x| x.sum()).sum::<usize>()
}

fn process_wf(workflow: &HashMap<String, Vec<Rule>>, wf_name: String, part: Part) -> Option<&str> {
    let rules = workflow.get(&wf_name).unwrap();
    for rule in rules.iter() {
        match rule.rate {
            Some('x') => match rule.op.unwrap() {
                Op::Sup => {
                    if part.x > rule.value.unwrap() {
                        return Some(&rule.dest);
                    }
                }
                Op::Inf => {
                    if part.x < rule.value.unwrap() {
                        return Some(&rule.dest);
                    }
                }
            },
            Some('m') => match rule.op.unwrap() {
                Op::Sup => {
                    if part.m > rule.value.unwrap() {
                        return Some(&rule.dest);
                    }
                }
                Op::Inf => {
                    if part.m < rule.value.unwrap() {
                        return Some(&rule.dest);
                    }
                }
            },
            Some('a') => match rule.op.unwrap() {
                Op::Sup => {
                    if part.a > rule.value.unwrap() {
                        return Some(&rule.dest);
                    }
                }
                Op::Inf => {
                    if part.a < rule.value.unwrap() {
                        return Some(&rule.dest);
                    }
                }
            },
            Some('s') => match rule.op.unwrap() {
                Op::Sup => {
                    if part.s > rule.value.unwrap() {
                        return Some(&rule.dest);
                    }
                }
                Op::Inf => {
                    if part.s < rule.value.unwrap() {
                        return Some(&rule.dest);
                    }
                }
            },
            Some(_) => panic!("Unknown rule rate: {}", rule.rate.unwrap()),
            None => {
                return Some(&rule.dest);
            }
        }
    }
    None
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
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
            "
        )));
        dbg!(&input);
        let answer = run(input);
        assert_eq!(answer, 19114);
    }
}
