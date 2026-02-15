use std::collections::HashMap;

use aoc_utils::Solution;
use winnow::ascii::{alphanumeric1, dec_uint, multispace0, multispace1, space1};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{Parser, Result, seq};

struct Day24;

impl Solution for Day24 {
    type Input<'a> = (
        HashMap<&'a str, Gate<'a>>,
        Vec<&'a str>,
        Vec<&'a str>,
        Vec<&'a str>,
    );
    type Output1 = u64;
    type Output2 = String;

    fn process(input: &str) -> Self::Input<'_> {
        let (values, gate_descriptions) = parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"));

        let mut gates: HashMap<&str, Gate> = HashMap::new();
        let mut xs: Vec<&str> = Vec::new();
        let mut ys: Vec<&str> = Vec::new();
        let mut zs: Vec<&str> = Vec::new();

        for (name, value) in values {
            gates.insert(name, Gate::Value(value == 1));

            if name.starts_with('x') {
                xs.push(name);
            } else if name.starts_with('y') {
                ys.push(name);
            }
        }

        for (a, op, b, out) in gate_descriptions {
            let gate = match op {
                "AND" => Gate::And(a, b),
                "OR" => Gate::Or(a, b),
                "XOR" => Gate::Xor(a, b),
                _ => panic!("Unknown gate type: {op}"),
            };
            gates.insert(out, gate);

            if out.starts_with('z') {
                zs.push(out);
            }
        }

        xs.sort_unstable();
        ys.sort_unstable();
        zs.sort_unstable();

        (gates, xs, ys, zs)
    }

    fn part1((gates, _xs, _ys, zs): &Self::Input<'_>) -> Self::Output1 {
        let mut result = 0;

        for &name in zs.iter().rev() {
            let value = u64::from(gates.get(name).unwrap().eval(gates));
            result = (result << 1) | value;
        }

        result
    }

    fn part2((gates, xs, ys, zs): &Self::Input<'_>) -> Self::Output2 {
        let graphviz_dot_file_content = create_graphviz_dot_file(gates, xs, ys);

        std::fs::write("circuit.dot", graphviz_dot_file_content).unwrap();
        std::process::Command::new("dot")
            .args(["-Tsvg", "-o", "circuit.svg", "circuit.dot"])
            .output()
            .expect("failed to execute process");

        // From analyzing the image visually
        let swaps = [
            ("qnw", "z15"),
            ("cqr", "z20"),
            ("ncd", "nfj"),
            ("vkg", "z37"),
        ];

        let swapped_gates = swap_entries(&mut gates.clone(), &swaps);
        assert!(is_valid_adder(&swapped_gates, xs, ys, zs), "Invalid adder");

        let mut ordered_swaps = swaps
            .into_iter()
            .flat_map(|(a, b)| vec![a, b])
            .collect::<Vec<_>>();
        ordered_swaps.sort_unstable();
        ordered_swaps.join(",")
    }
}

aoc_utils::run!(2024, 24, Day24);

type ParsedInput<'a> = (
    Vec<(&'a str, u64)>,
    Vec<(&'a str, &'a str, &'a str, &'a str)>,
);
fn parse_input<'a>(input: &mut &'a str) -> Result<ParsedInput<'a>> {
    terminated(
        separated_pair(
            separated(
                2..,
                separated_pair(alphanumeric1, ": ", dec_uint::<_, u64, _>),
                multispace1,
            ),
            multispace0,
            separated(
                1..,
                seq!(alphanumeric1, _: space1, alphanumeric1, _: space1, alphanumeric1, _: (space1, "->", space1), alphanumeric1),
                multispace1,
            ),
        ),
        multispace0,
    )
        .parse_next(input)
}

#[derive(Debug, Clone)]
enum Gate<'a> {
    Value(bool),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

impl Gate<'_> {
    fn eval(&self, gates: &HashMap<&str, Gate<'_>>) -> bool {
        match self {
            Self::Value(value) => *value,
            Self::And(a, b) => {
                let a = gates.get(a).unwrap().eval(gates);
                let b = gates.get(b).unwrap().eval(gates);
                a & b
            }
            Self::Or(a, b) => {
                let a = gates.get(a).unwrap().eval(gates);
                let b = gates.get(b).unwrap().eval(gates);
                a | b
            }
            Self::Xor(a, b) => {
                let a = gates.get(a).unwrap().eval(gates);
                let b = gates.get(b).unwrap().eval(gates);
                a ^ b
            }
        }
    }
}

fn create_graphviz_dot_file(gates: &HashMap<&str, Gate>, xs: &[&str], ys: &[&str]) -> String {
    let mut next_id = 0;
    let mut dot = String::new();

    dot.push_str("digraph {\n");

    for &name in xs {
        dot.push_str(&format!("  {name} [shape=oval]\n"));
    }

    for &name in ys {
        dot.push_str(&format!("  {name} [shape=oval]\n"));
    }

    for &name in gates.keys() {
        dot.push_str(&format!("  {name} [shape=oval]\n"));
    }

    for (&name, gate) in gates {
        match gate {
            Gate::Value(_) => {}
            Gate::And(a, b) => {
                let temp = format!("AND_{next_id:#03}");
                next_id += 1;
                dot.push_str(&format!("  {temp} [label=AND][shape=rectangle]\n",));
                dot.push_str(&format!("  {a} -> {temp}\n"));
                dot.push_str(&format!("  {b} -> {temp}\n"));
                dot.push_str(&format!("  {temp} -> {name}\n"));
            }
            Gate::Or(a, b) => {
                let temp = format!("OR_{next_id:#03}");
                next_id += 1;
                dot.push_str(&format!("  {temp} [label=OR][shape=rectangle]\n"));
                dot.push_str(&format!("  {a} -> {temp}\n"));
                dot.push_str(&format!("  {b} -> {temp}\n"));
                dot.push_str(&format!("  {temp} -> {name}\n"));
            }
            Gate::Xor(a, b) => {
                let temp = format!("XOR_{next_id:#03}");
                next_id += 1;
                dot.push_str(&format!("  {temp} [label=XOR][shape=rectangle]\n"));
                dot.push_str(&format!("  {a} -> {temp}\n"));
                dot.push_str(&format!("  {b} -> {temp}\n"));
                dot.push_str(&format!("  {temp} -> {name}\n"));
            }
        }
    }

    dot.push_str("}\n");

    dot
}

fn is_valid_full_adder<'a>(
    gates: &HashMap<&'a str, Gate<'a>>,
    x: &str,
    y: &str,
    z: &str,
    c: &mut &'a str,
) -> bool {
    let xy_xor = *gates
        .iter()
        .find(|(_name, gate)| {
            if let Gate::Xor(a, b) = gate
                && ((x == *a && y == *b) || (x == *b && y == *a))
            {
                return true;
            }
            false
        })
        .unwrap()
        .0;

    if !gates.iter().any(|(&name, gate)| {
        if let Gate::Xor(a, b) = gate
            && z == name
            && ((xy_xor == *a && c == b) || (xy_xor == *b && c == a))
        {
            return true;
        }
        false
    }) {
        return false;
    };

    let Some((&xy_c_and, _)) = gates.iter().find(|(_name, gate)| {
        if let Gate::And(a, b) = gate
            && ((xy_xor == *a && c == b) || (xy_xor == *b && c == a))
        {
            return true;
        }
        false
    }) else {
        return false;
    };

    let xy_and = *gates
        .iter()
        .find(|(_name, gate)| {
            if let Gate::And(a, b) = gate
                && ((x == *a && y == *b) || (x == *b && y == *a))
            {
                return true;
            }
            false
        })
        .unwrap()
        .0;

    *c = if let Some((&name, _)) = gates.iter().find(|(_name, gate)| {
        if let Gate::Or(a, b) = gate
            && ((xy_c_and == *a && xy_and == *b) || (xy_c_and == *b && xy_and == *a))
        {
            return true;
        }
        false
    }) {
        name
    } else {
        return false;
    };

    true
}

fn is_valid_adder(gates: &HashMap<&str, Gate<'_>>, xs: &[&str], ys: &[&str], zs: &[&str]) -> bool {
    // check that start is correct
    if !gates.iter().any(|(&name, gate)| {
        if let Gate::Xor(a, b) = gate
            && name == zs[0]
            && ((xs[0] == *a && ys[0] == *b) || (xs[0] == *b && ys[0] == *a))
        {
            return true;
        }
        false
    }) {
        return false;
    }

    let mut c = *gates
        .iter()
        .find(|(_name, gate)| {
            if let Gate::And(a, b) = gate
                && ((xs[0] == *a && ys[0] == *b) || (xs[0] == *b && ys[0] == *a))
            {
                return true;
            }
            false
        })
        .unwrap()
        .0;

    for i in 1..xs.len() {
        if !is_valid_full_adder(gates, xs[i], ys[i], zs[i], &mut c) {
            // Helpful for finding the spots on the svg to look at
            println!("Faulty at index {i}");
            return false;
        }
    }

    // check that end is correct
    if c != zs[zs.len() - 1] {
        return false;
    }

    true
}

fn swap_entries<'a>(
    gates: &mut HashMap<&'a str, Gate<'a>>,
    swaps: &[(&'a str, &'a str); 4],
) -> HashMap<&'a str, Gate<'a>> {
    let mut new_gates = gates.clone();

    for (s1, s2) in swaps {
        if let (Some(g1), Some(g2)) = (new_gates.remove(s1), new_gates.remove(s2)) {
            new_gates.insert(*s1, g2);
            new_gates.insert(*s2, g1);
        }
    }

    new_gates
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // Note: The automatic test input fetching retrieves incorrect data.
    aoc_utils::solution_tests!(2024, 24, Day24);

    #[rstest]
    fn part1_test1(data1: <Day24 as Solution>::Input<'_>) {
        assert_eq!(4, Day24::part1(&data1));
    }

    #[rstest]
    fn part1_test2(data2: <Day24 as Solution>::Input<'_>) {
        assert_eq!(2024, Day24::part1(&data2));
    }
}
