use reqwest::blocking::Client;
use rookie::firefox;
use std::fs;
use std::path::Path;

const CACHE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/cache");

fn get_session() -> Result<String, &'static str> {
    let cookies = firefox(Some(vec!["adventofcode.com".to_string()]))
        .map_err(|_| "Failed to get session cookie.")?;
    cookies
        .first()
        .map(|cookie| cookie.value.clone())
        .ok_or("Failed to get session cookie.")
}

fn fetch_text(url: &str) -> Result<String, String> {
    let session = get_session()?;
    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={session}"))
        .send()
        .map_err(|_| "Failed to send request.")?;

    if response.status() != 200 {
        return Err(format!("Received non-200 response: {}", response.status()));
    }

    response
        .text()
        .map_err(|_| "Failed to read response text.".into())
}

fn fetch_problem(year: u32, day: u32) -> Result<String, String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}");
    fetch_text(&url)
}

fn fetch_input(year: u32, day: u32) -> Result<String, String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    fetch_text(&url)
}

pub fn get_answers(year: u32, day: u32) -> Result<(Option<String>, Option<String>), String> {
    let cache_file = format!("{CACHE_DIR}/{year}-{day}-answers.txt");

    let mut answers = (None, None);

    if let Ok(contents) = fs::read_to_string(&cache_file) {
        let mut lines = contents.lines();
        if let Some(answer) = lines.next() {
            if !answer.is_empty() {
                answers.0 = Some(answer.to_string());
            }
        }
        if let Some(answer) = lines.next() {
            if !answer.is_empty() {
                answers.1 = Some(answer.to_string());
            }
        }
        return Ok(answers);
    }

    let contents = fetch_problem(year, day)?;
    let mut contents: &str = &contents;

    if let Some(index) = contents.find("Your puzzle answer was <code>") {
        contents = &contents[index + "Your puzzle answer was <code>".len()..];
        let end = contents.find("</code>.").unwrap();
        let answer = contents[..end].to_string();
        answers.0 = Some(answer);
    }

    if let Some(index) = contents.find("Your puzzle answer was <code>") {
        contents = &contents[index + "Your puzzle answer was <code>".len()..];
        let end = contents.find("</code>.").unwrap();
        let answer = contents[..end].to_string();
        answers.1 = Some(answer);
    }

    if contents.contains("Both parts of this puzzle are complete! They provide two gold stars: **")
    {
        let mut contents = String::new();
        if let Some(answer) = &answers.0 {
            contents.push_str(answer);
            contents.push('\n');
        }
        if let Some(answer) = &answers.1 {
            contents.push_str(answer);
            contents.push('\n');
        }
        fs::create_dir_all(CACHE_DIR).map_err(|_| "Failed to create cache directory.")?;
        fs::write(&cache_file, &contents).map_err(|_| "Failed to write cache file.")?;
    }

    Ok(answers)
}

pub fn get_input_file(year: u32, day: u32) -> Result<String, String> {
    let cache_file = format!("{CACHE_DIR}/{year}-{day}-input.txt");

    if Path::new(&cache_file).exists() {
        return Ok(cache_file);
    }

    let contents = fetch_input(year, day)?;

    fs::create_dir_all(CACHE_DIR).map_err(|_| "Failed to create cache directory.")?;
    fs::write(&cache_file, &contents).map_err(|_| "Failed to write cache file.")?;

    Ok(cache_file)
}
