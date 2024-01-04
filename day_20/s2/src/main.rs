use std::{collections::HashMap, ops::Range};

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
    // let (input, parts) = many1(parse_p_line)(input)?;

    let workflow = workflow
        .iter()
        .cloned()
        .collect::<HashMap<String, Vec<Rule>>>();
    let data = Data { workflow };
    Ok((input, data))
}
// fn parse_p_line(input: &str) -> IResult<&str, Part> {
//     let (input, _) = tag("{x=")(input)?;
//     let (input, x) = digit1(input)?;
//     let (input, _) = tag(",m=")(input)?;
//     let (input, m) = digit1(input)?;
//     let (input, _) = tag(",a=")(input)?;
//     let (input, a) = digit1(input)?;
//     let (input, _) = tag(",s=")(input)?;
//     let (input, s) = digit1(input)?;
//     let (input, _) = tag("}")(input)?;
//     let (input, _) = line_ending(input)?;
//     Ok((
//         input,
//         Part {
//             x: x.parse().unwrap(),
//             m: m.parse().unwrap(),
//             a: a.parse().unwrap(),
//             s: s.parse().unwrap(),
//         },
//     ))
// }

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
    // parts data are not needed anymore
    // parts: Vec<Part>,
}

#[derive(Debug, Clone)]
struct Rule {
    rate: Option<char>,
    op: Option<Op>,
    value: Option<usize>,
    dest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl Part {
    fn combinations(&self) -> usize {
        (self.x.len()) * (self.m.len()) * (self.a.len()) * (self.s.len())
    }
}

#[derive(Debug, Clone)]
struct WFStep {
    part: Part,
    wf_name: String,
    rule_index: usize,
}

impl Default for WFStep {
    fn default() -> Self {
        Self {
            part: Part {
                x: 1..4001,
                m: 1..4001,
                a: 1..4001,
                s: 1..4001,
            },
            wf_name: String::from("in"),
            rule_index: 0,
        }
    }
}

fn run(input: String) -> usize {
    let (_, data) = parse(&input).unwrap();
    dbg!(&data);

    let mut accepted = Vec::new();
    let mut wf_steps = Vec::new();
    wf_steps.push(WFStep::default());

    while let Some(wf_step) = wf_steps.pop() {
        let nwf_steps = process_wf_steps(&data.workflow, wf_step);

        for nwf_step in nwf_steps {
            match nwf_step {
                Some(nwf_step) => match nwf_step.wf_name.as_str() {
                    "A" => {
                        accepted.push(nwf_step.part);
                    }
                    "R" => {}
                    _ => {
                        wf_steps.push(nwf_step);
                    }
                },
                None => {
                    dbg!("We should not pass here unless a range in empty (start>end)");
                }
            }
        }
    }
    dbg!(&accepted);
    accepted.iter().map(|x| x.combinations()).sum::<usize>()
}

fn process_wf_steps(workflow: &HashMap<String, Vec<Rule>>, wf_step: WFStep) -> Vec<Option<WFStep>> {
    let rules = workflow.get(&wf_step.wf_name).unwrap();
    let rule = rules.get(wf_step.rule_index).unwrap();
    match rule.rate {
        Some(_) => process_rate_rule(&wf_step, rule),
        None => process_redirect_rule(&wf_step, rule),
    }
}

fn process_redirect_rule(wf_step: &WFStep, rule: &Rule) -> Vec<Option<WFStep>> {
    let mut nwf_steps = Vec::new();
    let mut nwf_step_ok = wf_step.clone();
    nwf_step_ok.wf_name = rule.dest.clone();
    nwf_step_ok.rule_index = 0;
    nwf_steps.push(Some(nwf_step_ok));
    nwf_steps
}

fn process_rate_rule(wf_step: &WFStep, rule: &Rule) -> Vec<Option<WFStep>> {
    let mut nwf_steps = Vec::new();
    let mut nwf_step_ok = wf_step.clone();
    let mut nwf_step_ko = wf_step.clone();

    // We return 2 x WFStep, one if the rule is ok and one if the rule is ko
    // get_range return a mutable reference to the impacted range
    let part_rate_ok = get_range(rule, &mut nwf_step_ok);
    let part_rate_ko = get_range(rule, &mut nwf_step_ko);

    // Initial range is 1..4001  note this is half open range 1 <= x < 4001
    // If we split the range in 2 with x<2000
    // we get 1..2000 and 2000..4001
    // If x > 2000:
    // we get 2001..4001 and 1..2001
    if rule.op.unwrap() == Op::Inf {
        let range = part_rate_ok.start..rule.value.unwrap();
        *part_rate_ok = range;
        let range = rule.value.unwrap()..part_rate_ko.end;
        *part_rate_ko = range;
    } else {
        let range = rule.value.unwrap() + 1..part_rate_ok.end;
        *part_rate_ok = range;
        let range = part_rate_ko.start..rule.value.unwrap() + 1;
        *part_rate_ko = range;
    }

    if Range::is_empty(part_rate_ok) {
        nwf_steps.push(None);
    } else {
        nwf_step_ok.wf_name = rule.dest.clone();
        nwf_step_ok.rule_index = 0;
        nwf_steps.push(Some(nwf_step_ok));
    }

    if Range::is_empty(part_rate_ko) {
        nwf_steps.push(None);
    } else {
        nwf_step_ko.rule_index += 1;
        nwf_steps.push(Some(nwf_step_ko));
    }
    nwf_steps
}

fn get_range<'a>(rule: &'a Rule, nwf_step: &'a mut WFStep) -> &'a mut Range<usize> {
    match rule.rate.unwrap() {
        'x' => &mut nwf_step.part.x,
        'm' => &mut nwf_step.part.m,
        'a' => &mut nwf_step.part.a,
        's' => &mut nwf_step.part.s,
        _ => panic!("Unknown rule rate: {}", rule.rate.unwrap()),
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
    fn test_combinations() {
        let part = Part {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        };
        let combinations = part.combinations();
        assert_eq!(combinations, 4000 * 4000 * 4000 * 4000);
    }
    //
    #[test]
    fn test_combinations2() {
        let part = Part {
            x: 1..2001,
            m: 2001..4001,
            a: 1..4001,
            s: 1..4001,
        };
        let combinations = part.combinations();
        assert_eq!(combinations, 2000 * 2000 * 4000 * 4000);
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
        assert_eq!(answer, 167409079868000);
    }
}
