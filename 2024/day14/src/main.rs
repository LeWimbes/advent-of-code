use aoc_utils::Solution;
use image::{ImageBuffer, Rgb};
use std::fs::create_dir;
use winnow::ascii::{dec_int, multispace0, multispace1};
use winnow::combinator::{delimited, preceded, repeat, separated_pair, terminated};
use winnow::{Parser, Result};

struct Day14;

impl Solution for Day14 {
    type Input<'a> = Vec<((i64, i64), (i64, i64))>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
    }

    fn part1(robots: &Self::Input<'_>) -> Self::Output1 {
        part1_parameterized(robots, 101, 103)
    }

    fn part2(robots: &Self::Input<'_>) -> Self::Output2 {
        if create_dir("lobby_states/").is_ok() {
            let width = 101;
            let height = 103;
            let mut changing_robots = robots.clone();

            write_lobby(&changing_robots, width, height, 0);

            for s in 1..10_000 {
                tick(&mut changing_robots, width, height);
                write_lobby(&changing_robots, width, height, s);
            }
        } else {
            println!(
                "'lobby_states' directory probably already exists. Delete it if you want to generate new images."
            );
        }

        // From analyzing the images visually
        6512
    }
}

aoc_utils::run!(2024, 14, Day14);

fn parse_input<'a>(input: &mut &str) -> Result<<Day14 as Solution>::Input<'a>> {
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

fn tick(robots: &mut <Day14 as Solution>::Input<'_>, width: i64, height: i64) {
    for (p, v) in robots {
        p.0 = (p.0 + v.0).rem_euclid(width);
        p.1 = (p.1 + v.1).rem_euclid(height);
    }
}

fn part1_parameterized(robots: &<Day14 as Solution>::Input<'_>, width: i64, height: i64) -> usize {
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

fn write_lobby(robots: &<Day14 as Solution>::Input<'_>, width: i64, height: i64, second: usize) {
    let mut imgbuf = ImageBuffer::new(width as u32, height as u32);

    for &((x, y), _) in robots {
        *imgbuf.get_pixel_mut(x as u32, y as u32) = Rgb([0, u8::MAX, 0]);
    }
    imgbuf
        .save(format!("lobby_states/{second:0>6}.png"))
        .expect("Couldn't write lobby!");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 14, Day14);

    #[rstest]
    fn part1_test(data1: <Day14 as Solution>::Input<'_>) {
        assert_eq!(12, part1_parameterized(&data1, 11, 7));
    }
}
