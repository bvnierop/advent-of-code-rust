use std::path::PathBuf;
use time::{OffsetDateTime, Month};
use crate::fs::{FileSystem, RealFileSystem, DryRunFileSystem};
use crate::aoc_client::{AdventOfCodeClient, HttpAdventOfCodeClient};
use std::process::Command;

// Public Interface
// ---------------

#[derive(Debug, Clone)]
pub enum YearOrDay {
    Year(u16),
    Day(u8),
}

/// Parse a string as either a year (≥2015) or day (1-25)
///
/// # Examples
/// ```
/// # use aoc::prep::parse_year_or_day;
/// assert!(matches!(parse_year_or_day("2024").unwrap(), YearOrDay::Year(2024)));
/// assert!(matches!(parse_year_or_day("1").unwrap(), YearOrDay::Day(1)));
/// assert!(parse_year_or_day("26").is_err());
/// ```
pub fn parse_year_or_day(arg: &str) -> Result<YearOrDay, String> {
    let num: u16 = arg.parse().map_err(|_| "Not a number".to_string())?;
    
    if num >= 2015 {
        Ok(YearOrDay::Year(num))
    } else if (1..=25).contains(&(num as u8)) {
        Ok(YearOrDay::Day(num as u8))
    } else {
        Err("Invalid year or day".to_string())
    }
}

/// Handle the prep command: prepare the environment for solving a problem
pub fn handle(first: Option<YearOrDay>, second: Option<YearOrDay>, dry_run: bool) {
    let (year, day) = extract_year_and_day(first, second);
    let (current_year, current_day) = get_current_advent();
    let year = year.unwrap_or(current_year);
    let day = day.unwrap_or(current_day);
    
    println!("{}Preparing environment for year {} day {}...", 
        if dry_run { "Would be " } else { "" },
        year, 
        day);
    
    let fs: &dyn FileSystem = if dry_run { &DryRunFileSystem } else { &RealFileSystem };
    
    let client = match HttpAdventOfCodeClient::new() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create AoC client: {}", e);
            return;
        }
    };
    
    if let Err(e) = create_files(year, day, fs, &client) {
        eprintln!("Failed to create files: {}", e);
    }
}

// Argument Handling
// ----------------

fn extract_year_and_day(first: Option<YearOrDay>, second: Option<YearOrDay>) -> (Option<u16>, Option<u8>) {
    match (first, second) {
        (None, None) => (None, None),
        (Some(YearOrDay::Year(y)), None) => (Some(y), None),
        (Some(YearOrDay::Day(d)), None) => (None, Some(d)),
        (Some(YearOrDay::Year(y)), Some(YearOrDay::Day(d))) => (Some(y), Some(d)),
        (Some(YearOrDay::Day(d)), Some(YearOrDay::Year(y))) => (Some(y), Some(d)),
        _ => (None, None), // Invalid combinations (year+year or day+day)
    }
}

fn get_current_advent() -> (u16, u8) {
    let now = OffsetDateTime::now_utc();
    let year = now.year() as u16;
    
    let day = if now.month() == Month::December && (1..=25).contains(&now.day()) {
        now.day() as u8
    } else {
        1
    };
    
    (year, day)
}

// File Creation
// ------------

const SOLUTION_TEMPLATE: &str = r###"pub fn solve_level1(input: &[&str]) -> String {
    todo!("Implement solution for level 1")
}

pub fn solve_level2(input: &[&str]) -> String {
    todo!("Implement solution for level 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
"#;

    #[test]
    fn test_level1() {
        let input: Vec<_> = EXAMPLE.lines().collect();
        assert_eq!(solve_level1(&input), "expected");
    }

    #[test]
    fn test_level2() {
        let input: Vec<_> = EXAMPLE.lines().collect();
        assert_eq!(solve_level2(&input), "expected");
    }
}"###;

fn create_files(year: u16, day: u8, fs: &dyn FileSystem, client: &dyn AdventOfCodeClient) -> std::io::Result<()> {
    let (statement, name) = match get_problem_info(year, day, client) {
        Ok((s, n)) => (s, n),
        Err(e) => {
            eprintln!("Failed to get problem info: {}", e);
            return Ok(());
        }
    };

    create_all_files(year, day, &name, &statement, fs, client)
}

fn create_all_files(year: u16, day: u8, name: &str, statement: &str, fs: &dyn FileSystem, client: &dyn AdventOfCodeClient) 
    -> std::io::Result<()> 
{
    create_file(fs, get_solution_paths(year, day, name), SOLUTION_TEMPLATE)?;
    
    if let Ok(org) = convert_html_to_org(statement) {
        create_file(fs, get_problem_paths(year, day, name), &org)?;
    }
    
    let (dir, sample_in, sample_out) = get_sample_paths(year, day);
    create_file(fs, (dir.clone(), sample_in), "")?;
    create_file(fs, (dir, sample_out), "\n\n")?;
    
    if let Ok(input) = client.get_problem_input(year, day) {
        create_file(fs, get_input_paths(year, day), &input)?;
    }
    
    Ok(())
}

fn create_file(fs: &dyn FileSystem, (dir, path): (PathBuf, PathBuf), contents: &str) -> std::io::Result<()> {
    if !fs.exists(&path) {
        fs.create_dir_all(&dir)?;
        fs.write_file(&path, contents)?;
    }
    Ok(())
}

fn get_problem_info(year: u16, day: u8, client: &dyn AdventOfCodeClient) 
    -> Result<(String, String), Box<dyn std::error::Error>> 
{
    let statement = client.get_problem_statement(year, day)?;
    let name = client.extract_problem_name(&statement)
        .unwrap_or_else(|_| "placeholder-name".to_string());
    
    Ok((statement, name))
}

// Path Generation
// --------------

struct FileConfig {
    base_dir: &'static str,
    extension: &'static str,
    name_format: NameFormat,
}

enum NameFormat {
    WithProblemName(String),  // {day}-{name}.{ext}
    Sample,                   // {day}-sample.{ext}
    Plain,                    // {day}.{ext}
}

fn get_file_path(year: u16, day: u8, config: FileConfig) -> (PathBuf, PathBuf) {
    let base_dir = PathBuf::from(config.base_dir).join(year.to_string());
    
    let filename = match config.name_format {
        NameFormat::WithProblemName(name) => {
            format!("{:02}-{}.{}", day, to_kebab_case(&name), config.extension)
        },
        NameFormat::Sample => format!("{:02}-sample.{}", day, config.extension),
        NameFormat::Plain => format!("{:02}.{}", day, config.extension),
    };
    
    (base_dir.clone(), base_dir.join(filename))
}

fn get_solution_paths(year: u16, day: u8, name: &str) -> (PathBuf, PathBuf) {
    get_file_path(year, day, FileConfig {
        base_dir: "src/solutions",
        extension: "rs",
        name_format: NameFormat::WithProblemName(name.to_string()),
    })
}

fn get_problem_paths(year: u16, day: u8, name: &str) -> (PathBuf, PathBuf) {
    get_file_path(year, day, FileConfig {
        base_dir: "problem",
        extension: "org",
        name_format: NameFormat::WithProblemName(name.to_string()),
    })
}

fn get_input_paths(year: u16, day: u8) -> (PathBuf, PathBuf) {
    get_file_path(year, day, FileConfig {
        base_dir: "input",
        extension: "in",
        name_format: NameFormat::Plain,
    })
}

fn get_sample_paths(year: u16, day: u8) -> (PathBuf, PathBuf, PathBuf) {
    let (dir, sample_in) = get_file_path(year, day, FileConfig {
        base_dir: "input",
        extension: "in",
        name_format: NameFormat::Sample,
    });
    
    let (_dir, sample_out) = get_file_path(year, day, FileConfig {
        base_dir: "input",
        extension: "out",
        name_format: NameFormat::Sample,
    });
    
    (dir, sample_in, sample_out)
}

// Utilities
// --------

fn convert_html_to_org(html: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut child = Command::new("pandoc")
        .arg("-f")
        .arg("html")
        .arg("-t")
        .arg("org")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        stdin.write_all(html.as_bytes())?;
    }
    
    let output = child.wait_with_output()?;
    
    if !output.status.success() {
        return Err("Pandoc conversion failed".into());
    }
    
    Ok(String::from_utf8(output.stdout)?)
}

fn to_kebab_case(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .replace("--", "-")
        .trim_matches('-')
        .to_string()
}

// Tests
// -----

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fs::MockFileSystem;
    use crate::aoc_client::FakeClient;

    #[test]
    fn test_year_day_parsing() {
        assert!(matches!(parse_year_or_day("2024").unwrap(), YearOrDay::Year(2024)));
        assert!(matches!(parse_year_or_day("1").unwrap(), YearOrDay::Day(1)));
        assert!(matches!(parse_year_or_day("25").unwrap(), YearOrDay::Day(25)));
        assert!(parse_year_or_day("26").is_err());
        assert!(parse_year_or_day("2014").is_err());
    }

    #[test]
    fn test_create_files() {
        let mut mock = MockFileSystem::new();
        
        // We only care that files are created with the right names
        mock.expect_exists()
            .returning(|path| path.to_str().unwrap().contains("placeholder"));
            
        mock.expect_create_dir_all()
            .returning(|_| Ok(()));
            
        mock.expect_write_file()
            .returning(|_, _| Ok(()));
        
        let client = FakeClient::new("test html", "Test Problem");
        create_files(2024, 1, &mock, &client).unwrap();
    }

    #[test]
    fn test_solution_paths() {
        let (_dir, file) = get_solution_paths(2024, 1, "Test Problem Name!");
        assert_eq!(file.file_name().unwrap(), "01-test-problem-name.rs");
    }

    #[test]
    fn test_problem_paths() {
        let (_dir, file) = get_problem_paths(2024, 1, "Test Problem Name!");
        assert_eq!(file.file_name().unwrap(), "01-test-problem-name.org");
    }

    #[test]
    fn test_sample_paths() {
        let (_dir, input, output) = get_sample_paths(2024, 1);
        assert_eq!(input.file_name().unwrap(), "01-sample.in");
        assert_eq!(output.file_name().unwrap(), "01-sample.out");
    }

    #[test]
    fn test_input_paths() {
        let (_dir, file) = get_input_paths(2024, 1);
        assert_eq!(file.file_name().unwrap(), "01.in");
    }
} 