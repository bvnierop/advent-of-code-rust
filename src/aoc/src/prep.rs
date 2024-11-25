use std::path::PathBuf;
use time::{OffsetDateTime, Month};
use crate::fs::{FileSystem, RealFileSystem, DryRunFileSystem};
use reqwest::blocking::Client;
use scraper::{Html, Selector};

#[derive(Debug, Clone)]
pub enum YearOrDay {
    Year(u16),
    Day(u8),
}

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

pub fn extract_year_and_day(first: Option<YearOrDay>, second: Option<YearOrDay>) -> (Option<u16>, Option<u8>) {
    match (first, second) {
        (None, None) => (None, None),
        (Some(YearOrDay::Year(y)), None) => (Some(y), None),
        (Some(YearOrDay::Day(d)), None) => (None, Some(d)),
        (Some(YearOrDay::Year(y)), Some(YearOrDay::Day(d))) => (Some(y), Some(d)),
        (Some(YearOrDay::Day(d)), Some(YearOrDay::Year(y))) => (Some(y), Some(d)),
        _ => (None, None), // Invalid combinations (year+year or day+day)
    }
}

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

fn get_problem_name(year: u16, day: u8) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let client = Client::new();
    let response = client.get(&url).send()?;
    let html = response.text()?;
    
    let document = Html::parse_document(&html);
    let selector = Selector::parse("article.day-desc > h2").unwrap();
    
    document.select(&selector)
        .next()
        .and_then(|h2| {
            h2.text()
                .collect::<String>()
                .split(':')
                .nth(1)
                .map(|s| s.trim().to_string())
        })
        .ok_or_else(|| "Could not find problem title".into())
}

fn get_solution_paths(year: u16, day: u8, name: &str) -> (PathBuf, PathBuf) {
    let solutions_dir = PathBuf::from("src")
        .join("solutions")
        .join(year.to_string());
    
    // Convert problem name to kebab case
    let name = name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .replace("--", "-")
        .trim_matches('-')
        .to_string();
        
    let filename = format!("{:02}-{}.rs", day, name);
    
    (solutions_dir.clone(), solutions_dir.join(filename))
}

fn create_solution_file(year: u16, day: u8, fs: &dyn FileSystem) -> std::io::Result<()> {
    let name = match get_problem_name(year, day) {
        Ok(name) => name,
        Err(e) => {
            eprintln!("Warning: Could not fetch problem name: {}", e);
            "placeholder-name".to_string()
        }
    };

    let (dir, path) = get_solution_paths(year, day, &name);

    if fs.exists(&path) {
        return Ok(());
    }

    fs.create_dir_all(&dir)?;
    fs.write_file(&path, SOLUTION_TEMPLATE)?;
    Ok(())
}

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
    
    if let Err(e) = create_solution_file(year, day, fs) {
        eprintln!("Failed to create solution file: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use crate::fs::MockFileSystem;

    #[test]
    fn test_year_day_parsing() {
        assert!(matches!(parse_year_or_day("2024").unwrap(), YearOrDay::Year(2024)));
        assert!(matches!(parse_year_or_day("1").unwrap(), YearOrDay::Day(1)));
        assert!(matches!(parse_year_or_day("25").unwrap(), YearOrDay::Day(25)));
        assert!(parse_year_or_day("26").is_err());
        assert!(parse_year_or_day("2014").is_err());
    }

    #[test]
    fn test_create_solution_file() {
        let year = 2024;
        let day = 1;
        let expected_dir = PathBuf::from("src/solutions/2024");
        let expected_file = expected_dir.join("01-placeholder-name.rs");
        
        let mut mock = MockFileSystem::new();
        mock.expect_create_dir_all()
            .with(eq(expected_dir))
            .times(1)
            .returning(|_| Ok(()));
        
        mock.expect_exists()
            .with(eq(expected_file.clone()))
            .times(1)
            .return_const(false);
            
        mock.expect_write_file()
            .with(eq(expected_file), eq(SOLUTION_TEMPLATE))
            .times(1)
            .returning(|_, _| Ok(()));
        
        create_solution_file(year, day, &mock).unwrap();
    }

    #[test]
    fn test_solution_paths() {
        let (_dir, file) = get_solution_paths(2024, 1, "Test Problem Name!");
        assert_eq!(file.file_name().unwrap(), "01-test-problem-name.rs");
    }
} 