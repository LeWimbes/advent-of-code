use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_utils::Solution;

struct Day09;

impl Solution for Day09 {
    type Input<'a> = (Vec<Coordinate>, Vec<(Reverse<i64>, usize, usize)>);
    type Output1 = i64;
    type Output2 = i64;

    fn process(input: &str) -> Self::Input<'_> {
        let coords = input
            .lines()
            .map(|line| {
                let mut iter = line.split(',').map(|num| num.parse::<i64>().unwrap());
                let (x, y) = (iter.next().unwrap(), iter.next().unwrap());
                Coordinate { x, y }
            })
            .collect::<Vec<_>>();

        let mut areas = BinaryHeap::with_capacity(coords.len() * (coords.len() - 1) / 2);
        for i in 0..coords.len() {
            for j in (i + 1)..coords.len() {
                let dist = coords[i].area(&coords[j]);
                areas.push((Reverse(dist), i, j));
            }
        }

        (coords, areas.into_sorted_vec())
    }

    fn part1((_, areas): &Self::Input<'_>) -> Self::Output1 {
        areas.first().unwrap().0.0
    }

    fn part2((coords, areas): &Self::Input<'_>) -> Self::Output2 {
        areas
            .iter()
            .find(|(_area, i, j)| {
                Coordinate::rectangle_in_polygon(&coords[*i], &coords[*j], coords)
            })
            .unwrap()
            .0
            .0
    }
}

aoc_utils::run!(2025, 9, Day09);

struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    /// Calculate the area of the rectangle defined by this coordinate and another coordinate
    fn area(&self, other: &Coordinate) -> i64 {
        let dx = self.x.abs_diff(other.x) as i64 + 1;
        let dy = self.y.abs_diff(other.y) as i64 + 1;
        dx * dy
    }

    /// Check if point is inside or on the polygon boundary
    fn in_or_on_polygon(&self, polygon: &[Coordinate]) -> bool {
        let mut inside = false;
        let (px, py) = (self.x, self.y);

        for (a, b) in polygon.iter().zip(polygon.iter().cycle().skip(1)) {
            let collinear = cross_product(a, b, self) == 0;
            if collinear {
                let in_bounds = px >= a.x.min(b.x)
                    && px <= a.x.max(b.x)
                    && py >= a.y.min(b.y)
                    && py <= a.y.max(b.y);
                if in_bounds {
                    return true; // On edge
                }
                continue; // Skip inside check for collinear edges
            }

            // Ray-casting for inside check
            // The coordinates must be on opposite sides of the horizontal ray
            if (a.y > py) != (b.y > py) {
                // x = a.x + t * (b.x - a.x)
                // y = a.y + t * (b.y - a.y)
                // Solve for t where y = py:
                // t = (py - a.y) / (b.y - a.y)
                let intersection_x = (b.x - a.x) * (py - a.y) / (b.y - a.y) + a.x;
                if px < intersection_x {
                    inside = !inside;
                }
            }
        }
        inside
    }

    /// Check if a rectangle (defined by two opposite corners) is fully inside the polygon
    fn rectangle_in_polygon(c1: &Coordinate, c2: &Coordinate, polygon: &[Coordinate]) -> bool {
        let min_x = c1.x.min(c2.x);
        let max_x = c1.x.max(c2.x);
        let min_y = c1.y.min(c2.y);
        let max_y = c1.y.max(c2.y);

        let corners = [
            Coordinate { x: min_x, y: min_y },
            Coordinate { x: max_x, y: min_y },
            Coordinate { x: max_x, y: max_y },
            Coordinate { x: min_x, y: max_y },
        ];

        // All corners must be in or on the polygon
        if corners.iter().any(|c| !c.in_or_on_polygon(polygon)) {
            return false;
        }

        // Check that no polygon edge crosses through the rectangle interior
        !polygon
            .iter()
            .zip(polygon.iter().cycle().skip(1))
            .filter(|(a, b)| {
                // Skip edges entirely outside the rectangle bounds
                !((a.x < min_x && b.x < min_x)
                    || (a.x > max_x && b.x > max_x)
                    || (a.y < min_y && b.y < min_y)
                    || (a.y > max_y && b.y > max_y))
            })
            .any(|(a, b)| Self::edge_crosses_rect_interior(a, b, min_x, max_x, min_y, max_y))
    }

    /// Check if edge AB crosses the interior of the rectangle defined by min/max x/y
    fn edge_crosses_rect_interior(
        a: &Coordinate,
        b: &Coordinate,
        min_x: i64,
        max_x: i64,
        min_y: i64,
        max_y: i64,
    ) -> bool {
        // If either endpoint is strictly inside the rectangle, it crosses
        let a_inside = a.x > min_x && a.x < max_x && a.y > min_y && a.y < max_y;
        let b_inside = b.x > min_x && b.x < max_x && b.y > min_y && b.y < max_y;
        if a_inside || b_inside {
            return true;
        }

        // Rectangle edges
        let rect_edges: [(i64, i64, i64, i64); 4] = [
            (min_x, min_y, max_x, min_y), // bottom
            (max_x, min_y, max_x, max_y), // right
            (min_x, max_y, max_x, max_y), // top
            (min_x, min_y, min_x, max_y), // left
        ];

        // Check if any rectangle edge crosses polygon segment AB
        rect_edges.iter().any(|&(cx, cy, dx, dy)| {
            Self::segments_cross(
                a,
                b,
                &Coordinate { x: cx, y: cy },
                &Coordinate { x: dx, y: dy },
            )
        })
    }

    /// Check if segment AB strictly crosses segment CD
    fn segments_cross(a: &Coordinate, b: &Coordinate, c: &Coordinate, d: &Coordinate) -> bool {
        // Orientation of A and B relative to line CD
        let d1 = cross_product(c, d, a);
        let d2 = cross_product(c, d, b);

        // Orientation of C and D relative to line AB
        let d3 = cross_product(a, b, c);
        let d4 = cross_product(a, b, d);

        // Check if signs differ (opposite sides)
        let ab_straddles_cd = (d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0);
        let cd_straddles_ab = (d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0);

        ab_straddles_cd && cd_straddles_ab
    }
}

/// Cross product of vectors OP and OQ
fn cross_product(o: &Coordinate, p: &Coordinate, q: &Coordinate) -> i64 {
    (p.x - o.x) * (q.y - o.y) - (p.y - o.y) * (q.x - o.x)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2025, 9, Day09);

    #[rstest]
    fn part1_test(data1: <Day09 as Solution>::Input<'_>) {
        assert_eq!(50, Day09::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day09 as Solution>::Input<'_>) {
        assert_eq!(24, Day09::part2(&data1));
    }
}
