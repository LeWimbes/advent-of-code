use image::{ImageBuffer, Rgb};
use std::fs::create_dir;
use winnow::ascii::{dec_int, multispace0, multispace1};
use winnow::combinator::{delimited, preceded, repeat, separated_pair, terminated};
use winnow::{PResult, Parser};

type ParsedInput = Vec<((i64, i64), (i64, i64))>;
type ProcessedInput = Vec<((i64, i64), (i64, i64))>;

fn main() {
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data, 101, 103));
    part2(&data, 101, 103);
}

fn parse_input(input: &mut &str) -> PResult<ParsedInput> {
    terminated(
        repeat(
            0..,
            (
                preceded("p=", separated_pair(dec_int, ',', dec_int)),
                delimited(" v=", separated_pair(dec_int, ',', dec_int), multispace1),
            ),
        ),
        multispace0,
    )
    .parse_next(input)
}

fn process_input(input: &str) -> ProcessedInput {
    parse_input
        .parse(input)
        .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
}

fn tick(robots: &mut ProcessedInput, width: i64, height: i64) {
    for (p, v) in robots {
        p.0 = (p.0 + v.0).rem_euclid(width);
        p.1 = (p.1 + v.1).rem_euclid(height);
    }
}

fn part1(robots: &ProcessedInput, width: i64, height: i64) -> usize {
    let mut robots = robots.clone();

    for _ in 0..100 {
        tick(&mut robots, width, height);
    }

    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut quadrants = (0, 0, 0, 0);

    for ((x, y), _) in robots {
        if x == mid_x || y == mid_y {
            continue;
        }

        let quadrant = match (x < mid_x, y < mid_y) {
            (true, true) => &mut quadrants.0,
            (false, true) => &mut quadrants.1,
            (true, false) => &mut quadrants.2,
            (false, false) => &mut quadrants.3,
        };

        *quadrant += 1;
    }

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

fn write_lobby(robots: &ProcessedInput, width: i64, height: i64, second: usize) {
    let mut imgbuf = ImageBuffer::new(width as u32, height as u32);

    for &((x, y), _) in robots {
        *imgbuf.get_pixel_mut(x as u32, y as u32) = Rgb([0, u8::MAX, 0]);
    }
    imgbuf
        .save(format!("lobby_states/{second:0>6}.png"))
        .expect("Couldn't write lobby!");
}

fn part2(robots: &ProcessedInput, width: i64, height: i64) {
    let mut changing_robots = robots.clone();

    create_dir("lobby_states/").expect("Couldn't create directory!");

    write_lobby(&changing_robots, width, height, 0);

    for s in 1..10_000 {
        tick(&mut changing_robots, width, height);
        write_lobby(&changing_robots, width, height, s);
    }
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn data() -> ProcessedInput {
        let input = include_str!("input_test.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: ProcessedInput) {
        assert_eq!(part1(&data, 11, 7), 12);
    }
}
