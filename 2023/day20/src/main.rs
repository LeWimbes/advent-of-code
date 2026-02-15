use std::collections::{HashMap, VecDeque};

use aoc_utils::Solution;
use num::Integer;

struct Day20;

impl Solution for Day20 {
    type Input<'a> = HashMap<&'a str, Module<'a>>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
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
                    .or_insert(Vec::new())
                    .push(*name);
            }
        }

        for (name, origins) in module_origins {
            if let Some(module) = modules.get_mut(&name) {
                match &mut module.m_type {
                    ModuleType::Broadcaster => {}
                    ModuleType::FlipFlop { state: _state } => {}
                    ModuleType::Conjunction { inputs } => {
                        *inputs = origins.iter().map(|name| (*name, false)).collect()
                    }
                }
            }
        }

        modules
    }

    fn part1(modules: &Self::Input<'_>) -> Self::Output1 {
        let mut modules = modules.clone();
        count_pulses_and_observe(&mut modules, Some(1000), None)
    }

    fn part2(modules: &Self::Input<'_>) -> Self::Output2 {
        // rx has one parent (&jm)
        // jm has four parents (&sg, &lm, &dh, &db)
        // These four grandparents of rx all have only one input.
        // It is therefore sufficient to look for incoming low pulses on these modules.
        // By finding the cycles for these four pulses, we can use LCM to calculate the desired value.

        let parent: &str = modules
            .iter()
            .find_map(|(name, module)| module.outputs.contains(&"rx").then_some(*name))
            .unwrap();
        let grandparents: Vec<&str> = modules
            .iter()
            .filter_map(|(name, module)| module.outputs.contains(&parent).then_some(*name))
            .collect();

        grandparents
            .iter()
            .map(|name| {
                let mut modules = modules.clone();
                count_pulses_and_observe(&mut modules, None, Some(name))
            })
            .fold(1, |acc, steps| steps.lcm(&acc))
    }
}

aoc_utils::run!(2023, 20, Day20);

#[derive(Debug, Clone, Eq, PartialEq)]
enum ModuleType<'a> {
    Broadcaster,
    FlipFlop { state: bool },
    Conjunction { inputs: HashMap<&'a str, bool> },
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Module<'a> {
    m_type: ModuleType<'a>,
    name: &'a str,
    outputs: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn create_broadcaster(outputs: Vec<&'a str>) -> Self {
        Module {
            m_type: ModuleType::Broadcaster,
            name: "broadcaster",
            outputs,
        }
    }

    fn create_flip_flop(name: &'a str, outputs: Vec<&'a str>) -> Self {
        Module {
            m_type: ModuleType::FlipFlop { state: false },
            name,
            outputs,
        }
    }

    fn create_conjunction(name: &'a str, outputs: Vec<&'a str>) -> Self {
        Module {
            m_type: ModuleType::Conjunction {
                inputs: HashMap::new(),
            },
            name,
            outputs,
        }
    }

    fn receive(&mut self, source: &'a str, pulse: bool) -> Vec<(&'a str, bool, &'a str)> {
        match &mut self.m_type {
            ModuleType::Broadcaster => self
                .outputs
                .iter()
                .map(|&name| (self.name, pulse, name))
                .collect(),

            ModuleType::FlipFlop { state } => {
                if pulse {
                    Vec::new()
                } else {
                    *state = !*state;
                    self.outputs
                        .iter()
                        .map(|&name| (self.name, *state, name))
                        .collect()
                }
            }

            ModuleType::Conjunction { inputs } => {
                *inputs.get_mut(source).unwrap() = pulse;

                let pulse_to_send = !inputs.values().all(|last_pulse| *last_pulse);

                self.outputs
                    .iter()
                    .map(|&name| (self.name, pulse_to_send, name))
                    .collect()
            }
        }
    }
}

fn simulate_btn_press_and_observe(
    modules: &mut <Day20 as Solution>::Input<'_>,
    to_observe: Option<&str>,
) -> ((usize, usize), bool) {
    let mut pulses = (0, 0);
    let should_observe = to_observe.is_some();
    let mut observed = false;

    let mut to_propagate = VecDeque::new();
    let signal_from_btn = ("btn", false, "broadcaster");
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

        if let Some(module) = modules.get_mut(module_name) {
            let new_signal = module.receive(src, pulse);
            for signal in new_signal {
                to_propagate.push_back(signal);
            }
        }
    }

    (pulses, observed)
}

fn count_pulses_and_observe(
    modules: &mut <Day20 as Solution>::Input<'_>,
    btn_presses: Option<usize>,
    to_observe: Option<&str>,
) -> usize {
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // Note: The automatic test input fetching retrieves incorrect data.
    aoc_utils::solution_tests!(2023, 20, Day20);

    #[rstest]
    fn part1_test1(data1: <Day20 as Solution>::Input<'_>) {
        assert_eq!(32_000_000, Day20::part1(&data1));
    }

    #[rstest]
    fn part1_test2(data2: <Day20 as Solution>::Input<'_>) {
        assert_eq!(11_687_500, Day20::part1(&data2));
    }
}
