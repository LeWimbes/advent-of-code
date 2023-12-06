use regex::Regex;

fn main() {
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &str) -> Vec<Vec<(u32, u32, u32)>> {
    input.lines().map(|line| line.split(";").map(|set| {
        let re = Regex::new(r"(?<count>\d+) (?<color>(red|green|blue))").unwrap();

        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for caps in re.captures_iter(set) {
            match &caps["color"] {
                "red" => red = caps["count"].parse().unwrap(),
                "green" => green = caps["count"].parse().unwrap(),
                "blue" => blue = caps["count"].parse().unwrap(),
                _ => {}
            }
        }

        (red, green, blue)
    }).collect()).collect()
}

fn part1(games: &Vec<Vec<(u32, u32, u32)>>) -> u32 {
    let max: (u32, u32, u32) = (12, 13, 14);
    games.iter().enumerate().filter_map(|(id, game)| {
        if game.iter().all(|set| set.0 <= max.0 && set.1 <= max.1 && set.2 <= max.2) {
            Some(id as u32 + 1)
        } else {
            None
        }
    }).sum()
}

fn part2(games: &Vec<Vec<(u32, u32, u32)>>) -> u32 {
    games.iter().map(|game| {
        let minimums = game.iter()
            .fold((0, 0, 0), |acc, set| (acc.0.max(set.0), acc.1.max(set.1), acc.2.max(set.2)));
        minimums.0 * minimums.1 * minimums.2
    }).sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn data() -> Vec<Vec<(u32, u32, u32)>> {
        let input = include_str!("input_test.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<Vec<(u32, u32, u32)>>) {
        assert_eq!(part1(&data), 8);
    }

    #[rstest]
    fn part2_test(data: Vec<Vec<(u32, u32, u32)>>) {
        assert_eq!(part2(&data), 2286);
    }
}
