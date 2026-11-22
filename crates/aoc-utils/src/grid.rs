//! Two-dimensional grids, positions in them and the four directions.
//!
//! `x` is the column and `y` the row, with `y` growing downwards like in the puzzle input.

use std::fmt;
use std::ops::{Index, IndexMut};

/// A position in a [`Grid`].
///
/// Positions are ordered like reading the grid: by row first, then by column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    #[must_use]
    pub const fn new(x: usize, y: usize) -> Self {
        Pos { x, y }
    }

    /// The position moved by `dx` and `dy`, or `None` if it would be negative.
    #[must_use]
    pub fn offset(self, dx: isize, dy: isize) -> Option<Pos> {
        Some(Pos::new(
            self.x.checked_add_signed(dx)?,
            self.y.checked_add_signed(dy)?,
        ))
    }

    /// The neighbouring position in `dir`, or `None` if it would be negative.
    #[must_use]
    pub fn step(self, dir: Direction) -> Option<Pos> {
        let (dx, dy) = dir.offset();
        self.offset(dx, dy)
    }

    /// The Manhattan distance between both positions.
    #[must_use]
    pub fn manhattan(self, other: Pos) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

/// One of the four directions, in clockwise order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// All four directions, clockwise starting at north.
    pub const ALL4: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    /// The change in `x` and `y` when moving one step in this direction.
    #[must_use]
    pub const fn offset(self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }

    #[must_use]
    pub const fn turn_right(self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    #[must_use]
    pub const fn turn_left(self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    #[must_use]
    pub const fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

/// The offsets `(dx, dy)` of the eight surrounding positions, clockwise starting at north.
pub const OFFSETS8: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

/// A rectangular grid, stored row by row.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    /// A grid with every cell set to `value`.
    #[must_use]
    pub fn new(width: usize, height: usize, value: T) -> Self
    where
        T: Clone,
    {
        Grid {
            width,
            height,
            cells: vec![value; width * height],
        }
    }

    /// Parse a grid with one row per line, converting each character with `f`.
    ///
    /// Empty lines are skipped, so a trailing newline doesn't add a row.
    ///
    /// # Panics
    ///
    /// Panics if the rows don't all have the same length.
    pub fn parse(input: &str, mut f: impl FnMut(char) -> T) -> Self {
        let mut width = None;
        let mut height = 0;
        let mut cells = Vec::with_capacity(input.len());

        for line in input.lines().filter(|line| !line.is_empty()) {
            let len_before = cells.len();
            cells.extend(line.chars().map(&mut f));
            let row_width = cells.len() - len_before;
            let width = *width.get_or_insert(row_width);
            assert_eq!(
                row_width, width,
                "row {height} has {row_width} cells, but the first row has {width}"
            );
            height += 1;
        }

        Grid {
            width: width.unwrap_or(0),
            height,
            cells,
        }
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub fn contains(&self, pos: Pos) -> bool {
        pos.x < self.width && pos.y < self.height
    }

    /// The index of `pos` when numbering the cells row by row, from 0 to `width * height - 1`.
    #[must_use]
    pub fn flat_index(&self, pos: Pos) -> usize {
        pos.y * self.width + pos.x
    }

    #[must_use]
    pub fn get(&self, pos: Pos) -> Option<&T> {
        self.contains(pos)
            .then(|| &self.cells[self.flat_index(pos)])
    }

    #[must_use]
    pub fn get_mut(&mut self, pos: Pos) -> Option<&mut T> {
        let index = self.flat_index(pos);
        self.contains(pos).then(|| &mut self.cells[index])
    }

    /// The neighbouring position in `dir`, or `None` if it's outside the grid.
    #[must_use]
    pub fn step(&self, pos: Pos, dir: Direction) -> Option<Pos> {
        // Wrapping turns negative coordinates into huge ones, which `contains` rejects.
        // That's faster than `Pos::offset`, which checks both coordinates separately.
        let (dx, dy) = dir.offset();
        let next = Pos::new(pos.x.wrapping_add_signed(dx), pos.y.wrapping_add_signed(dy));
        self.contains(next).then_some(next)
    }

    /// The orthogonal neighbours of `pos` that are inside the grid.
    pub fn neighbors4(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        Direction::ALL4
            .into_iter()
            .filter_map(move |dir| self.step(pos, dir))
    }

    /// The orthogonal and diagonal neighbours of `pos` that are inside the grid.
    pub fn neighbors8(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        // Wrapping turns negative coordinates into huge ones, which `contains` rejects.
        // That's faster than `Pos::offset`, which checks both coordinates separately.
        OFFSETS8
            .into_iter()
            .map(move |(dx, dy)| {
                Pos::new(pos.x.wrapping_add_signed(dx), pos.y.wrapping_add_signed(dy))
            })
            .filter(|&next| self.contains(next))
    }

    /// All positions, row by row.
    pub fn positions(&self) -> impl Iterator<Item = Pos> + use<T> {
        let (width, height) = (self.width, self.height);
        (0..height).flat_map(move |y| (0..width).map(move |x| Pos::new(x, y)))
    }

    /// All positions with their cells, row by row.
    pub fn iter(&self) -> impl Iterator<Item = (Pos, &T)> {
        self.positions().zip(&self.cells)
    }

    /// The rows of the grid, from top to bottom.
    pub fn rows(&self) -> impl DoubleEndedIterator<Item = &[T]> + ExactSizeIterator {
        // `max(1)` because `chunks` panics for 0, and an empty grid has no cells anyway
        self.cells.chunks(self.width.max(1))
    }

    /// The cells of row `y`, from left to right.
    ///
    /// # Panics
    ///
    /// Panics if `y` is outside the grid.
    #[must_use]
    pub fn row(&self, y: usize) -> &[T] {
        assert!(y < self.height, "row {y} is outside the grid");
        &self.cells[y * self.width..(y + 1) * self.width]
    }

    /// The cells of column `x`, from top to bottom.
    ///
    /// # Panics
    ///
    /// Panics if `x` is outside the grid.
    pub fn column(&self, x: usize) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        assert!(x < self.width, "column {x} is outside the grid");
        self.cells[x..].iter().step_by(self.width)
    }

    /// The columns of the grid, from left to right.
    pub fn columns(
        &self,
    ) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = &T> + ExactSizeIterator>
    + ExactSizeIterator {
        (0..self.width).map(|x| self.column(x))
    }

    /// The first position, row by row, whose cell equals `value`.
    #[must_use]
    pub fn find(&self, value: &T) -> Option<Pos>
    where
        T: PartialEq,
    {
        self.iter()
            .find_map(|(pos, cell)| (cell == value).then_some(pos))
    }

    /// A grid of the same size with `f` applied to every cell.
    #[must_use]
    pub fn map<U>(&self, f: impl FnMut(&T) -> U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            cells: self.cells.iter().map(f).collect(),
        }
    }
}

impl<T> Index<Pos> for Grid<T> {
    type Output = T;

    /// # Panics
    ///
    /// Panics if `pos` is outside the grid.
    fn index(&self, pos: Pos) -> &T {
        self.get(pos)
            .unwrap_or_else(|| panic!("{pos:?} is outside the grid"))
    }
}

impl<T> IndexMut<Pos> for Grid<T> {
    /// # Panics
    ///
    /// Panics if `pos` is outside the grid.
    fn index_mut(&mut self, pos: Pos) -> &mut T {
        self.get_mut(pos)
            .unwrap_or_else(|| panic!("{pos:?} is outside the grid"))
    }
}

impl fmt::Display for Grid<char> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows() {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "ab.\n.cd\n";

    #[test]
    fn parses_rows() {
        let grid = Grid::parse(INPUT, |c| c);
        assert_eq!((grid.width(), grid.height()), (3, 2));
        assert_eq!(grid[Pos::new(1, 0)], 'b');
        assert_eq!(grid[Pos::new(2, 1)], 'd');
        assert_eq!(grid.to_string(), INPUT);
    }

    #[test]
    fn accesses_rows_and_columns() {
        let grid = Grid::parse(INPUT, |c| c);
        assert_eq!(grid.row(1), ['.', 'c', 'd']);
        assert_eq!(grid.column(1).collect::<String>(), "bc");
        assert_eq!(grid.column(2).rev().collect::<String>(), "d.");
        assert_eq!(grid.rows().next_back(), Some(&['.', 'c', 'd'][..]));
        assert_eq!(
            grid.columns()
                .map(Iterator::collect)
                .collect::<Vec<String>>(),
            ["a.", "bc", ".d"]
        );
        assert_eq!(grid.flat_index(Pos::new(1, 1)), 4);
    }

    #[test]
    #[should_panic(expected = "row 1 has 2 cells, but the first row has 3")]
    fn rejects_ragged_rows() {
        Grid::parse("abc\nde\n", |c| c);
    }

    #[test]
    fn checks_bounds() {
        let grid = Grid::parse(INPUT, |c| c);
        assert_eq!(grid.get(Pos::new(3, 0)), None);
        assert_eq!(grid.get(Pos::new(0, 2)), None);
        assert_eq!(grid.step(Pos::new(0, 0), Direction::North), None);
        assert_eq!(grid.step(Pos::new(2, 0), Direction::East), None);
        assert_eq!(
            grid.step(Pos::new(0, 0), Direction::South),
            Some(Pos::new(0, 1))
        );
    }

    #[test]
    fn finds_neighbors_inside_the_grid() {
        let grid = Grid::new(3, 3, ());
        assert_eq!(grid.neighbors4(Pos::new(1, 1)).count(), 4);
        assert_eq!(grid.neighbors8(Pos::new(1, 1)).count(), 8);
        assert_eq!(
            grid.neighbors4(Pos::new(0, 0)).collect::<Vec<_>>(),
            [Pos::new(1, 0), Pos::new(0, 1)]
        );
        assert_eq!(grid.neighbors8(Pos::new(2, 2)).count(), 3);
    }

    #[test]
    fn iterates_row_by_row() {
        let grid = Grid::parse(INPUT, |c| c);
        assert_eq!(grid.find(&'c'), Some(Pos::new(1, 1)));
        assert_eq!(grid.find(&'x'), None);
        assert_eq!(grid.iter().map(|(_, &c)| c).collect::<String>(), "ab..cd");
        assert!(grid.positions().is_sorted());
    }

    #[test]
    fn turns() {
        for dir in Direction::ALL4 {
            assert_eq!(dir.turn_right().turn_left(), dir);
            assert_eq!(dir.turn_right().turn_right(), dir.opposite());
            let (dx, dy) = dir.offset();
            assert_eq!(dir.opposite().offset(), (-dx, -dy));
        }
    }
}
