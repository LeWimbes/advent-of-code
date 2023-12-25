use std::collections::{HashMap, VecDeque};

use num::Integer;

fn main() {
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop { state: bool },
    Conjunction { inputs: HashMap<&'a str, bool> },
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Module<'a> {
    module_type: ModuleType<'a>,
    name: &'a str,
    outputs: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn create_broadcaster(outputs: Vec<&'a str>) -> Self {
        Module { module_type: ModuleType::Broadcaster, name: "broadcaster", outputs }
    }

    fn create_flip_flop(name: &'a str, outputs: Vec<&'a str>) -> Self {
        Module { module_type: ModuleType::FlipFlop { state: false }, name, outputs }
    }

    fn create_conjunction(name: &'a str, outputs: Vec<&'a str>) -> Self {
        Module { module_type: ModuleType::Conjunction { inputs: HashMap::new() }, name, outputs }
    }

    fn receive(&mut self, source: &str, pulse: bool) -> Vec<(String, bool, String)> {
        match &mut self.module_type {
            ModuleType::Broadcaster => self.outputs.iter()
                .map(|&name| (self.name.to_string(), pulse, name.to_string()))
                .collect(),

            ModuleType::FlipFlop { state } => {
                if pulse {
                    vec![]
                } else {
                    *state = !*state;
                    self.outputs.iter()
                        .map(|&name| (self.name.to_string(), *state, name.to_string()))
                        .collect()
                }
            }

            ModuleType::Conjunction { inputs } => {
                *inputs.get_mut(source).unwrap() = pulse;

                let pulse_to_send = !inputs.values().all(|last_pulse| *last_pulse);

                self.outputs.iter()
                    .map(|&name| (self.name.to_string(), pulse_to_send, name.to_string()))
                    .collect()
            }
        }
    }
}

fn process_input(input: &str) -> HashMap<&str, Module> {
    let mut modules: HashMap<&str, Module> = input
        .lines()
        .map(|line| {
            let (module_name, outputs) = line.split_once(" -> ").unwrap();
            let outputs: Vec<&str> = outputs.split(", ").collect();
            let module = if let Some(name) = module_name.strip_prefix('%') {
                Module::create_flip_flop(name, outputs)
            } else if let Some(name) = module_name.strip_prefix('&') {
                Module::create_conjunction(name, outputs)
            } else {
                Module::create_broadcaster(outputs)
            };
            (module.name, module)
        })
        .collect();

    let mut module_origins = HashMap::new();
    for (name, module) in &modules {
        for dest in &module.outputs {
            module_origins
                .entry(*dest)
                .or_insert(vec![])
                .push(*name);
        }
    }

    for (name, origins) in module_origins {
        if let Some(module) = modules.get_mut(&name) {
            match &mut module.module_type {
                ModuleType::Broadcaster => {}
                ModuleType::FlipFlop { state: _state } => {}
                ModuleType::Conjunction { inputs } =>
                    *inputs = origins.iter().map(|name| (*name, false)).collect(),
            }
        }
    }

    modules
}

fn simulate_btn_press_and_observe(modules: &mut HashMap<&str, Module>, to_observe: Option<&str>) -> ((usize, usize), bool) {
    let mut pulses = (0, 0);
    let should_observe = to_observe.is_some();
    let mut observed = false;

    let mut to_propagate = VecDeque::new();
    let signal_from_btn = ("btn".to_string(), false, "broadcaster".to_string());
    to_propagate.push_back(signal_from_btn);

    while let Some((src, pulse, module_name)) = to_propagate.pop_front() {
        if pulse {
            pulses.1 += 1;
        } else {
            pulses.0 += 1;
        }
        if should_observe && !pulse && module_name == to_observe.unwrap() {
            observed = true;
        }

        if let Some(module) = modules.get_mut(module_name.as_str()) {
            let new_signal = module.receive(src.as_str(), pulse);
            for signal in new_signal {
                to_propagate.push_back(signal);
            }
        }
    }

    (pulses, observed)
}

fn count_pulses_and_observe(modules: &mut HashMap<&str, Module>, btn_presses: Option<usize>, to_observe: Option<&str>) -> usize {
    let mut pulses = (0, 0);

    for p in 1..=btn_presses.unwrap_or(usize::MAX) {
        let (added_pulses, observed) = simulate_btn_press_and_observe(modules, to_observe);
        if observed {
            return p;
        }
        pulses.0 += added_pulses.0;
        pulses.1 += added_pulses.1;
    }

    pulses.0 * pulses.1
}

fn part1(modules: &HashMap<&str, Module>) -> usize {
    let mut modules = modules.clone();
    count_pulses_and_observe(&mut modules, Some(1000), None)
}

fn part2(modules: &HashMap<&str, Module>) -> usize {
    // rx has one parent (&jm)
    // jm has four parents (&sg, &lm, &dh, &db)
    // These four grandparents of rx all have only one input.
    // It is therefore sufficient to look for incoming low pulses on these modules.
    // By finding the cycles for these four pulses, we can use LCM to calculate the desired value.

    let parent: &str = modules.iter()
        .find_map(|(name, module)| {
            module.outputs.contains(&"rx").then_some(*name)
        }).unwrap();
    let grandparents: Vec<&str> = modules.iter().filter_map(|(name, module)| {
        module.outputs.contains(&parent).then_some(*name)
    }).collect();

    grandparents.iter().map(|name| {
        let mut modules = modules.clone();
        count_pulses_and_observe(&mut modules, None, Some(name))
    }).fold(1, |acc, steps| steps.lcm(&acc))
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn data1<'a>() -> HashMap<&'a str, Module<'a>> {
        let input = include_str!("input_test1.txt");
        process_input(input)
    }

    #[fixture]
    fn data2<'a>() -> HashMap<&'a str, Module<'a>> {
        let input = include_str!("input_test2.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test1(data1: HashMap<&str, Module>) {
        assert_eq!(part1(&data1), 32000000);
    }

    #[rstest]
    fn part1_test2(data2: HashMap<&str, Module>) {
        assert_eq!(part1(&data2), 11687500);
    }
}
