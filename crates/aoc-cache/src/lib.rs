//! Caching utilities for `AoC` inputs and answers.

use ratelimit::Ratelimiter;
use rookie::firefox;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;
use thiserror::Error;

/// Errors that can occur when fetching or caching `AoC` data.
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("failed to create cache directory: {0}")]
    CreateCacheDir(#[source] io::Error),
    #[error("failed to write to cache: {0}")]
    WriteCache(#[source] io::Error),
    #[error("failed to get session cookie from browser")]
    SessionNotFound,
    #[error("HTTP request failed: {0}")]
    HttpRequest(String),
    #[error("received HTTP status {0}")]
    HttpStatus(u16),
    #[error("failed to read response body: {0}")]
    HttpBody(#[source] io::Error),
    #[error("failed to parse problem page: {0}")]
    ParseError(String),
}

/// Result type for cache operations.
pub type Result<T> = std::result::Result<T, CacheError>;

fn get_cache_dir() -> &'static PathBuf {
    static CACHE_DIR: OnceLock<PathBuf> = OnceLock::new();
    CACHE_DIR.get_or_init(|| {
        std::env::var("AOC_CACHE_DIR").map_or_else(
            |_| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/cache")),
            PathBuf::from,
        )
    })
}

fn get_cache_file(file_name: &str) -> Result<PathBuf> {
    let cache_dir = get_cache_dir();
    fs::create_dir_all(cache_dir).map_err(CacheError::CreateCacheDir)?;
    Ok(cache_dir.join(file_name))
}

fn get_rate_limiter() -> &'static Ratelimiter {
    static LIMITER: OnceLock<Ratelimiter> = OnceLock::new();
    LIMITER.get_or_init(|| {
        Ratelimiter::builder(1, Duration::from_secs(1))
            .max_tokens(1)
            .build()
            .unwrap()
    })
}

fn get_session() -> Result<String> {
    let cookies = firefox(Some(vec!["adventofcode.com".to_string()]))
        .map_err(|_| CacheError::SessionNotFound)?;
    cookies
        .first()
        .map(|cookie| cookie.value.clone())
        .ok_or(CacheError::SessionNotFound)
}

fn fetch_text(url: &str) -> Result<String> {
    while let Err(sleep) = get_rate_limiter().try_wait() {
        std::thread::sleep(sleep);
    }

    let session = get_session()?;
    let mut response = ureq::get(url)
        .header("Cookie", &format!("session={session}"))
        .call()
        .map_err(|e| CacheError::HttpRequest(e.to_string()))?;

    if response.status() != 200 {
        return Err(CacheError::HttpStatus(response.status().as_u16()));
    }

    response
        .body_mut()
        .read_to_string()
        .map_err(|e| CacheError::HttpBody(io::Error::other(e)))
}

fn fetch_problem(year: u32, day: u32) -> Result<String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}");
    fetch_text(&url)
}

fn fetch_input(year: u32, day: u32) -> Result<String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    fetch_text(&url)
}

fn fetch_answers(year: u32, day: u32) -> Result<(Option<String>, Option<String>)> {
    let contents = fetch_problem(year, day)?;

    let mut answers = (None, None);
    let mut contents: &str = &contents;

    if let Some(index) = contents.find("Your puzzle answer was <code>") {
        contents = &contents[index + "Your puzzle answer was <code>".len()..];
        let end = contents
            .find("</code>.")
            .ok_or_else(|| CacheError::ParseError("missing end of first answer".into()))?;
        answers.0 = Some(contents[..end].to_string());
    }

    if let Some(index) = contents.find("Your puzzle answer was <code>") {
        contents = &contents[index + "Your puzzle answer was <code>".len()..];
        let end = contents
            .find("</code>.")
            .ok_or_else(|| CacheError::ParseError("missing end of second answer".into()))?;
        answers.1 = Some(contents[..end].to_string());
    }

    Ok(answers)
}

fn fetch_example_inputs(year: u32, day: u32) -> Result<Vec<String>> {
    let problem_html = fetch_problem(year, day)?;

    let mut examples = Vec::new();
    let search_patterns = ["For example", "for example", "another example"];
    let mut search_start = 0;

    while search_start < problem_html.len() {
        let next_match = search_patterns
            .iter()
            .filter_map(|pattern| {
                problem_html[search_start..]
                    .find(pattern)
                    .map(|pos| (pos, pattern.len()))
            })
            .min_by_key(|(pos, _)| *pos);

        let Some((rel_pos, pattern_len)) = next_match else {
            break;
        };

        search_start = search_start + rel_pos + pattern_len;

        let Some(pre_start) = problem_html[search_start..]
            .find('\n')
            .map(|next_newline| search_start + next_newline + 1)
        else {
            continue;
        };
        if !problem_html[pre_start..].starts_with("<pre><code>") {
            continue;
        }

        let pre_start = pre_start + "<pre><code>".len();
        if let Some(pre_end) = problem_html[pre_start..].find("</code></pre>") {
            let pre_end = pre_start + pre_end;
            let example_input = &problem_html[pre_start..pre_end];
            let cleaned_input = example_input
                .replace("<em>", "")
                .replace("</em>", "")
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">");
            examples.push(cleaned_input);
            search_start = pre_end + "</code></pre>".len();
        } else {
            break;
        }
    }
    Ok(examples)
}

/// Get the path to the cached input file for a given year/day.
///
/// If the file doesn't exist, it will be fetched.
///
/// # Errors
///
/// Returns an error if
/// - the cache directory can't be created,
/// - or the input can't be fetched.
pub fn get_input_file(year: u32, day: u32) -> Result<String> {
    let cache_path = get_cache_file(&format!("{year}-{day}-input.txt"))?;

    if cache_path.exists() {
        return Ok(cache_path.to_string_lossy().into_owned());
    }

    let contents = fetch_input(year, day)?;
    fs::write(&cache_path, &contents).map_err(CacheError::WriteCache)?;

    Ok(cache_path.to_string_lossy().into_owned())
}

/// Get the submitted answers for a given year/day.
///
/// Returns a tuple of (`part1_answer`, `part2_answer`),
/// where each is `None` if that part hasn't been submitted yet.
///
/// # Errors
///
/// Returns an error if
/// - the cache directory can't be created,
/// - or answers can't be fetched.
pub fn get_answers(year: u32, day: u32) -> Result<(Option<String>, Option<String>)> {
    let cache_path = get_cache_file(&format!("{year}-{day}-answers.txt"))?;

    // Try to read from cache first
    if let Ok(contents) = fs::read_to_string(&cache_path) {
        let mut lines = contents.lines();
        let part1 = lines.next().filter(|s| !s.is_empty()).map(String::from);
        let part2 = lines.next().filter(|s| !s.is_empty()).map(String::from);
        return Ok((part1, part2));
    }

    // Fetch from website
    let answers = fetch_answers(year, day)?;

    // Only cache if both answers are present
    if answers.0.is_some() && answers.1.is_some() {
        let contents = format!(
            "{}\n{}\n",
            answers.0.as_deref().unwrap_or(""),
            answers.1.as_deref().unwrap_or("")
        );
        fs::write(&cache_path, &contents).map_err(CacheError::WriteCache)?;
    }

    Ok(answers)
}

/// Get the paths to cached example input files for a given year/day.
///
/// If the files don't exist, examples will be parsed from the problem page.
///
/// # Errors
///
/// Returns an error if
/// - the cache directory can't be created,
/// - or examples can't be fetched.
pub fn get_example_input_files(year: u32, day: u32) -> Result<Vec<String>> {
    let metadata_path = get_cache_file(&format!("{year}-{day}-example-inputs.txt"))?;

    // Check if we have cached example files
    if let Ok(count_str) = fs::read_to_string(&metadata_path)
        && let Ok(count) = count_str.trim().parse::<usize>()
    {
        let cache_files: Result<Vec<PathBuf>> = (1..=count)
            .map(|idx| get_cache_file(&format!("{year}-{day}-example-input{idx}.txt")))
            .collect();
        let cache_files = cache_files?;

        if cache_files.iter().all(|f| f.exists()) {
            return Ok(cache_files
                .into_iter()
                .map(|p| p.to_string_lossy().into_owned())
                .collect());
        }
    }

    // Fetch examples from website
    let examples = fetch_example_inputs(year, day)?;
    let mut cache_files = Vec::new();

    for (idx, content) in examples.iter().enumerate() {
        let cache_file = get_cache_file(&format!("{year}-{day}-example-input{}.txt", idx + 1))?;
        fs::write(&cache_file, content).map_err(CacheError::WriteCache)?;
        cache_files.push(cache_file.to_string_lossy().into_owned());
    }

    // Write metadata file with count
    fs::write(&metadata_path, examples.len().to_string()).map_err(CacheError::WriteCache)?;

    Ok(cache_files)
}
