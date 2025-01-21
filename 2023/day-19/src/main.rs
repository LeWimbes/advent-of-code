use std::collections::HashMap;
use std::ops::Range;
use std::result::Result;

use regex::Regex;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 19);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Category {
    X,
    M,
    A,
    S,
}

impl TryFrom<&str> for Category {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x" => Ok(Category::X),
            "m" => Ok(Category::M),
            "a" => Ok(Category::A),
            "s" => Ok(Category::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct PartRange {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}

fn len(range: &Range<u64>) -> u64 {
    if range.start > range.end {
        0
    } else {
        range.end - range.start
    }
}

impl PartRange {
    fn contains(&self, part: &Part) -> bool {
        self.x.contains(&part.x)
            && self.m.contains(&part.m)
            && self.a.contains(&part.a)
            && self.s.contains(&part.s)
    }

    fn combinations(&self) -> u64 {
        len(&self.x) * len(&self.m) * len(&self.a) * len(&self.s)
    }

    fn get(&self, category: Category) -> &Range<u64> {
        match category {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }

    fn replace(&self, category: Category, range: Range<u64>) -> Self {
        match category {
            Category::X => PartRange {
                x: range,
                m: self.m.clone(),
                a: self.a.clone(),
                s: self.s.clone(),
            },
            Category::M => PartRange {
                x: self.x.clone(),
                m: range,
                a: self.a.clone(),
                s: self.s.clone(),
            },
            Category::A => PartRange {
                x: self.x.clone(),
                m: self.m.clone(),
                a: range,
                s: self.s.clone(),
            },
            Category::S => PartRange {
                x: self.x.clone(),
                m: self.m.clone(),
                a: self.a.clone(),
                s: range,
            },
        }
    }

    fn empty() -> Self {
        PartRange {
            x: 0..0,
            m: 0..0,
            a: 0..0,
            s: 0..0,
        }
    }

    fn full() -> Self {
        PartRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ResultType {
    Accept,
    Reject,
    Redirect(String),
}

impl From<&str> for ResultType {
    fn from(value: &str) -> Self {
        match value {
            "A" => ResultType::Accept,
            "R" => ResultType::Reject,
            _ => ResultType::Redirect(value.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Comparator {
    LT,
    GT,
}

impl TryFrom<&str> for Comparator {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "<" => Ok(Comparator::LT),
            ">" => Ok(Comparator::GT),
            _ => Err(()),
        }
    }
}

impl Comparator {
    fn split(self, range: &Range<u64>, pivot: u64) -> (Range<u64>, Range<u64>) {
        match self {
            Comparator::LT => (range.start..pivot, pivot..range.end),
            Comparator::GT => (pivot + 1..range.end, range.start..pivot + 1),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Rule {
    Complex(Category, Comparator, u64, ResultType),
    Simple(ResultType),
}

impl Rule {
    fn from(
        category: Option<&str>,
        comparator: Option<&str>,
        value: Option<&str>,
        result: &str,
    ) -> Self {
        if category.is_some() && comparator.is_some() && value.is_some() {
            let category = category.unwrap().try_into().expect("Unknown category!");
            let comparator = comparator.unwrap().try_into().expect("Unknown comparator!");
            let value = value.unwrap().parse().expect("Not an u64!");
            Rule::Complex(category, comparator, value, ResultType::from(result))
        } else {
            Rule::Simple(ResultType::from(result))
        }
    }

    fn apply(&self, range: &PartRange) -> ((&ResultType, PartRange), PartRange) {
        match self {
            Rule::Complex(cat, comp, val, res) => {
                let parts = comp.split(range.get(*cat), *val);
                (
                    (res, range.replace(*cat, parts.0)),
                    range.replace(*cat, parts.1),
                )
            }
            Rule::Simple(res) => ((res, range.clone()), PartRange::empty()),
        }
    }
}

struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn apply(&self, range: &PartRange) -> (Vec<PartRange>, Vec<(&String, PartRange)>) {
        let mut accept = Vec::new();
        let mut redirect = Vec::new();

        let mut remaining = range.clone();
        for rule in &self.rules {
            let result = rule.apply(&remaining);
            match result.0 .0 {
                ResultType::Accept => accept.push(result.0 .1),
                ResultType::Reject => {}
                ResultType::Redirect(label) => redirect.push((label, result.0 .1)),
            }
            remaining = result.1;
        }

        (accept, redirect)
    }
}

fn compile(workflows: &HashMap<String, Workflow>) -> Vec<PartRange> {
    let mut accepted = Vec::new();

    let mut remaining = vec![("in".to_string(), PartRange::full())];

    while let Some((label, range)) = remaining.pop() {
        let result = workflows[&label].apply(&range);
        accepted.extend(
            result
                .0
                .iter()
                .filter_map(|range| (range.combinations() != 0).then_some(range.clone())),
        );
        remaining.extend(result.1.iter().filter_map(|(label, range)| {
            (range.combinations() != 0).then_some(((*label).clone(), range.clone()))
        }));
    }

    accepted
}

fn process_input(input: &'static str) -> (Vec<PartRange>, Vec<Part>) {
    let input_parts = input.split_once("\n\n").unwrap();
    let re_workflows = Regex::new(r"(?<name>[a-z]+)\{(?<rules>.+)}").unwrap();
    let re_rule = Regex::new(r"((?<category>[xmas])(?<comparator>[<>])(?<value>\d+):(?<result>([a-z]+|[AR])))|(?<simple>([a-z]+|[AR]))").unwrap();
    let re_parts = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)}").unwrap();

    let workflows: HashMap<String, Workflow> = re_workflows
        .captures_iter(input_parts.0)
        .map(|caps| {
            let name = caps["name"].to_string();
            let rules: Vec<Rule> = caps["rules"]
                .split(',')
                .map(|rule| {
                    let caps = re_rule.captures(rule).unwrap();
                    Rule::from(
                        caps.name("category").map(|m| m.as_str()),
                        caps.name("comparator").map(|m| m.as_str()),
                        caps.name("value").map(|m| m.as_str()),
                        caps.name("result")
                            .or(caps.name("simple"))
                            .map(|m| m.as_str())
                            .unwrap(),
                    )
                })
                .collect();

            (name, Workflow { rules })
        })
        .collect();

    let accepted = compile(&workflows);

    let parts: Vec<Part> = re_parts
        .captures_iter(input_parts.1)
        .map(|caps| Part {
            x: caps["x"].parse().unwrap(),
            m: caps["m"].parse().unwrap(),
            a: caps["a"].parse().unwrap(),
            s: caps["s"].parse().unwrap(),
        })
        .collect();

    (accepted, parts)
}

fn part1(accepted_parts: &(Vec<PartRange>, Vec<Part>)) -> u64 {
    let (accepted, parts) = accepted_parts;

    parts
        .iter()
        .filter(|part| accepted.iter().any(|range| range.contains(part)))
        .map(Part::sum)
        .sum()
}

fn part2(accepted_parts: &(Vec<PartRange>, Vec<Part>)) -> u64 {
    let (accepted, _parts) = accepted_parts;

    accepted.iter().map(PartRange::combinations).sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> (Vec<PartRange>, Vec<Part>) {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: (Vec<PartRange>, Vec<Part>)) {
        assert_eq!(part1(&data), 19114);
    }

    #[rstest]
    fn part2_test(data: (Vec<PartRange>, Vec<Part>)) {
        assert_eq!(part2(&data), 167_409_079_868_000);
    }
}
