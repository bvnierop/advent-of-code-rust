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

    create_all_files(year, day, &name, &statement, fs)
}

fn get_problem_info(year: u16, day: u8, client: &dyn AdventOfCodeClient) 
    -> Result<(String, String), Box<dyn std::error::Error>> 
{
    let statement = client.get_problem_statement(year, day)?;
    let name = client.extract_problem_name(&statement)
        .unwrap_or_else(|_| "placeholder-name".to_string());
    
    Ok((statement, name))
}

fn create_all_files(year: u16, day: u8, name: &str, statement: &str, fs: &dyn FileSystem) 
    -> std::io::Result<()> 
{
    create_solution_file(year, day, name, fs)?;
    create_problem_file(year, day, name, statement, fs)?;
    create_sample_files(year, day, fs)?;
    Ok(())
}

fn create_solution_file(year: u16, day: u8, name: &str, fs: &dyn FileSystem) -> std::io::Result<()> {
    let (dir, path) = get_solution_paths(year, day, name);
    if !fs.exists(&path) {
        fs.create_dir_all(&dir)?;
        fs.write_file(&path, SOLUTION_TEMPLATE)?;
        println!("Created solution file: {}", path.display());
    }
    Ok(())
}

fn create_problem_file(year: u16, day: u8, name: &str, html: &str, fs: &dyn FileSystem) -> std::io::Result<()> {
    let (dir, path) = get_problem_paths(year, day, name);
    if !fs.exists(&path) {
        match convert_html_to_org(html) {
            Ok(org) => {
                fs.create_dir_all(&dir)?;
                fs.write_file(&path, &org)?;
                println!("Created problem statement: {}", path.display());
            }
            Err(e) => eprintln!("Failed to convert problem statement: {}", e),
        }
    }
    Ok(())
}

fn create_sample_files(year: u16, day: u8, fs: &dyn FileSystem) -> std::io::Result<()> {
    let (dir, sample_in, sample_out) = get_sample_paths(year, day);
    
    if !fs.exists(&sample_in) {
        fs.create_dir_all(&dir)?;
        fs.write_file(&sample_in, "")?;
        println!("Created sample input file: {}", sample_in.display());
    }
    
    if !fs.exists(&sample_out) {
        fs.create_dir_all(&dir)?;
        fs.write_file(&sample_out, "\n\n")?; // Space for both level 1 and 2 answers
        println!("Created sample output file: {}", sample_out.display());
    }
    
    Ok(())
}

// Path Generation
// --------------

fn get_solution_paths(year: u16, day: u8, name: &str) -> (PathBuf, PathBuf) {
    let solutions_dir = PathBuf::from("src")
        .join("solutions")
        .join(year.to_string());
    
    let filename = format!("{:02}-{}.rs", day, to_kebab_case(name));
    
    (solutions_dir.clone(), solutions_dir.join(filename))
}

fn get_problem_paths(year: u16, day: u8, name: &str) -> (PathBuf, PathBuf) {
    let problems_dir = PathBuf::from("problem")
        .join(year.to_string());
    
    let filename = format!("{:02}-{}.org", day, to_kebab_case(name));
    
    (problems_dir.clone(), problems_dir.join(filename))
}

fn get_sample_paths(year: u16, day: u8) -> (PathBuf, PathBuf, PathBuf) {
    let input_dir = PathBuf::from("input")
        .join(year.to_string());
    
    let sample_in = input_dir.join(format!("{:02}-sample.in", day));
    let sample_out = input_dir.join(format!("{:02}-sample.out", day));
    
    (input_dir, sample_in, sample_out)
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
} 