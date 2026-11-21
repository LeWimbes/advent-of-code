//! Fetching data from the Advent of Code website.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rookie::firefox;

use crate::{CacheError, Result};

/// Minimum time between two requests, across all processes sharing the cache.
const MIN_REQUEST_INTERVAL: Duration = Duration::from_secs(1);

/// Identifies this tool to the Advent of Code servers.
const USER_AGENT: &str = "github.com/LeWimbes/advent-of-code (aoc-cache)";

/// Exclusive access to the website, shared by all processes using the same cache directory.
///
/// Macros run inside separate rustc processes, and rust-analyzer may expand them at the same time as cargo.
/// An in-process rate limiter can't coordinate them,
/// so this uses a lock file that also stores the time of the last request.
/// The lock is released when this is dropped.
pub(crate) struct RequestLock {
    file: File,
}

impl RequestLock {
    /// Block until no other process is accessing the website.
    pub(crate) fn acquire(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(CacheError::Lock)?;
        }
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(CacheError::Lock)?;
        file.lock().map_err(CacheError::Lock)?;
        Ok(Self { file })
    }

    /// Fetch a page, waiting first if the previous request was too recent.
    pub(crate) fn fetch(&mut self, url: &str) -> Result<String> {
        if let Some(elapsed) = self.time_since_last_request()
            && let Some(remaining) = MIN_REQUEST_INTERVAL.checked_sub(elapsed)
        {
            thread::sleep(remaining);
        }

        let result = fetch_text(url);
        self.record_request()?;
        result
    }

    fn time_since_last_request(&mut self) -> Option<Duration> {
        let mut contents = String::new();
        self.file.seek(SeekFrom::Start(0)).ok()?;
        self.file.read_to_string(&mut contents).ok()?;
        let millis = contents.trim().parse().ok()?;
        let last = UNIX_EPOCH + Duration::from_millis(millis);
        // A timestamp in the future (clock changes) counts as just now.
        Some(SystemTime::now().duration_since(last).unwrap_or_default())
    }

    fn record_request(&mut self) -> Result<()> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        self.file.set_len(0).map_err(CacheError::Lock)?;
        self.file
            .seek(SeekFrom::Start(0))
            .map_err(CacheError::Lock)?;
        write!(self.file, "{now}").map_err(CacheError::Lock)
    }
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
    let session = get_session()?;
    let mut response = ureq::get(url)
        .header("Cookie", &format!("session={session}"))
        .header("User-Agent", USER_AGENT)
        .call()
        .map_err(|e| CacheError::HttpRequest(e.to_string()))?;

    if response.status() != 200 {
        return Err(CacheError::HttpStatus(response.status().as_u16()));
    }

    response
        .body_mut()
        .read_to_string()
        .map_err(|e| CacheError::HttpBody(e.to_string()))
}
