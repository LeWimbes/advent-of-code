//! Fetches and caches puzzle data from the Advent of Code website.
//!
//! Everything is stored in the cache directory:
//!
//! ```text
//! cache/
//! ├── .last-request        # Lock and timestamp for the rate limit
//! └── 2024/day10/
//!     ├── input.txt        # The puzzle input
//!     ├── puzzle.html      # The puzzle page, containing the answers and examples
//!     └── examples/01.txt  # Every code block on the puzzle page, in order
//! ```
//!
//! Each file is only fetched once.
//! The puzzle page is fetched again only when it lacks data that is asked for:
//! a code block or answer that only appears once a part is solved.

mod extract;
mod fetch;

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use fetch::RequestLock;
use thiserror::Error;

/// Errors that can occur when fetching or caching `AoC` data.
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("failed to read from cache: {0}")]
    ReadCache(#[source] io::Error),
    #[error("failed to write to cache: {0}")]
    WriteCache(#[source] io::Error),
    #[error("failed to lock the cache for fetching: {0}")]
    Lock(#[source] io::Error),
    #[error("failed to get session cookie from browser")]
    SessionNotFound,
    #[error("HTTP request failed: {0}")]
    HttpRequest(String),
    #[error("received HTTP status {0}")]
    HttpStatus(u16),
    #[error("failed to read response body: {0}")]
    HttpBody(String),
    #[error("example indices start at 1")]
    InvalidExampleIndex,
    #[error("the puzzle page has no code block {index}, only {available}")]
    ExampleNotFound { index: usize, available: usize },
}

/// Result type for cache operations.
pub type Result<T> = std::result::Result<T, CacheError>;

/// Whether a day is the last of its year, which only has one part.
///
/// Until 2024, every year had 25 days.
/// Since 2025, there are 12.
#[must_use]
pub fn is_last_day(year: u32, day: u32) -> bool {
    if year <= 2024 { day == 25 } else { day == 12 }
}

static CACHE_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    // crates/aoc-cache -> repository root
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .join("cache")
});

fn day_dir(year: u32, day: u32) -> PathBuf {
    CACHE_DIR
        .join(year.to_string())
        .join(format!("day{day:02}"))
}

/// Get the path to the cached input for a given year and day, fetching it if needed.
///
/// # Errors
///
/// Returns an error if the input isn't cached and can't be fetched.
pub fn get_input_file(year: u32, day: u32) -> Result<String> {
    let path = day_dir(year, day).join("input.txt");
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    fetch_cached(&path, &url, |_| false)?;
    Ok(path_string(&path))
}

/// Get the paths to the given code blocks on the puzzle page, as example inputs.
///
/// `indices` start at 1 and refer to all code blocks on the page, in order.
/// The cached files in `examples/` can be used to find the right indices.
///
/// # Errors
///
/// Returns an error if
/// - an index is 0 or the page doesn't have that many code blocks,
/// - or the puzzle page isn't cached and can't be fetched.
pub fn get_example_files(year: u32, day: u32, indices: &[usize]) -> Result<Vec<String>> {
    if indices.contains(&0) {
        return Err(CacheError::InvalidExampleIndex);
    }
    let needed = indices.iter().copied().max().unwrap_or(0);

    // A block that's missing might be in part 2, which appears once part 1 is solved.
    let page = fetch_puzzle_page(year, day, |page| extract::code_blocks(page).len() < needed)?;
    let blocks = extract::code_blocks(&page);
    if needed > blocks.len() {
        return Err(CacheError::ExampleNotFound {
            index: needed,
            available: blocks.len(),
        });
    }

    // Write all blocks, so they can be browsed to pick the right indices.
    let examples_dir = day_dir(year, day).join("examples");
    let paths: Vec<PathBuf> = (1..=blocks.len())
        .map(|index| examples_dir.join(format!("{index:02}.txt")))
        .collect();
    for (path, block) in paths.iter().zip(&blocks) {
        write_if_changed(path, block)?;
    }

    Ok(indices
        .iter()
        .map(|&index| path_string(&paths[index - 1]))
        .collect())
}

/// Get the submitted answers for a given year and day.
///
/// Returns a tuple of (`part1_answer`, `part2_answer`),
/// where each is `None` if that part hasn't been solved yet.
///
/// # Errors
///
/// Returns an error if the puzzle page isn't cached or lacks answers, and can't be fetched.
pub fn get_answers(year: u32, day: u32) -> Result<(Option<String>, Option<String>)> {
    let expected = if is_last_day(year, day) { 1 } else { 2 };
    let page = fetch_puzzle_page(year, day, |page| extract::answers(page).len() < expected)?;
    let mut answers = extract::answers(&page).into_iter();
    Ok((answers.next(), answers.next()))
}

fn fetch_puzzle_page(year: u32, day: u32, is_outdated: impl Fn(&str) -> bool) -> Result<String> {
    let path = day_dir(year, day).join("puzzle.html");
    let url = format!("https://adventofcode.com/{year}/day/{day}");
    fetch_cached(&path, &url, is_outdated)
}

/// Read a cached file, fetching it first if it's missing or outdated.
///
/// If it's still outdated after fetching, the fetched version is returned anyway.
fn fetch_cached(path: &Path, url: &str, is_outdated: impl Fn(&str) -> bool) -> Result<String> {
    if let Some(contents) = read_cached(path)?
        && !is_outdated(&contents)
    {
        return Ok(contents);
    }

    let mut lock = RequestLock::acquire(&CACHE_DIR.join(".last-request"))?;

    // Another process might have fetched it while waiting for the lock.
    if let Some(contents) = read_cached(path)?
        && !is_outdated(&contents)
    {
        return Ok(contents);
    }

    let contents = lock.fetch(url)?;
    write_atomic(path, &contents)?;
    Ok(contents)
}

fn path_string(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

fn read_cached(path: &Path) -> Result<Option<String>> {
    match fs::read_to_string(path) {
        Ok(contents) => Ok(Some(contents)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(CacheError::ReadCache(e)),
    }
}

/// Write a file without touching it if the contents are the same.
///
/// The macros include these files, so touching them would trigger rebuilds.
fn write_if_changed(path: &Path, contents: &str) -> Result<()> {
    if read_cached(path)?.as_deref() == Some(contents) {
        return Ok(());
    }
    write_atomic(path, contents)
}

/// Write a file so that other processes never see it half-written.
fn write_atomic(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(CacheError::WriteCache)?;
    }
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(format!(".{}.tmp", std::process::id()));
    fs::write(&tmp, contents).map_err(CacheError::WriteCache)?;
    fs::rename(&tmp, path).map_err(CacheError::WriteCache)
}
