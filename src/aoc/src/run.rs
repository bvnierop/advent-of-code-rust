use std::path::{Path, PathBuf};
use std::time::Instant;
use std::fs;
use aoc_core::Solver;

#[derive(Debug)]
pub struct RunConfig {
    pub year: Option<u16>,
    pub day: Option<u8>,
    pub input_file: Option<String>,
    pub level: Option<u8>,
    pub solver: Option<String>,
}

/// Handle the run command: execute solution(s) for a given problem
pub fn handle(config: RunConfig) -> Result<(), Box<dyn std::error::Error>> {
    let (year, day) = get_year_and_day(config.year, config.day)?;
    let input_path = resolve_input_path(year, day, config.input_file.as_deref())?;
    let input = read_input(&input_path)?;
    let output_path = input_path.with_extension("out");
    let expected_output = fs::read_to_string(&output_path).ok();

    let solvers = discover_solvers(year, day)?;
    let filtered_solvers = filter_solvers(solvers, config.level, config.solver.as_deref());

    for solver in filtered_solvers {
        run_solver(&solver, &input, expected_output.as_deref())?;
    }

    Ok(())
}

fn get_year_and_day(year: Option<u16>, day: Option<u8>) -> Result<(u16, u8), String> {
    let now = time::OffsetDateTime::now_utc();
    let year = year.unwrap_or(now.year() as u16);
    let day = day.unwrap_or_else(|| {
        if now.month() == time::Month::December && (1..=25).contains(&now.day()) {
            now.day() as u8
        } else {
            1
        }
    });

    if year < 2015 {
        return Err("Year must be 2015 or later".into());
    }
    if !(1..=25).contains(&day) {
        return Err("Day must be between 1 and 25".into());
    }

    Ok((year, day))
}

fn resolve_input_path(year: u16, day: u8, input_suffix: Option<&str>) -> Result<PathBuf, String> {
    let base = PathBuf::from("input").join(year.to_string());
    let filename = match input_suffix {
        Some(suffix) => format!("{:02}-{}.in", day, suffix),
        None => format!("{:02}.in", day),
    };
    
    let path = base.join(filename);
    if !path.exists() {
        return Err(format!("Input file not found: {}", path.display()));
    }
    
    Ok(path)
}

fn read_input(path: &Path) -> Result<Vec<String>, std::io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content.lines().map(String::from).collect())
}

fn discover_solvers(year: u16, day: u8) -> Result<Vec<Solver>, String> {
    let solvers = aoc_core::discover_solvers(year, day);
    if solvers.is_empty() {
        Err(format!("No solvers found for year {} day {}", year, day))
    } else {
        Ok(solvers)
    }
}

fn filter_solvers(
    solvers: Vec<Solver>,
    level: Option<u8>,
    solver_name: Option<&str>,
) -> Vec<Solver> {
    solvers
        .into_iter()
        .filter(|s| level.map_or(true, |l| s.level == l))
        .filter(|s| solver_name.map_or(true, |name| s.name == name))
        .collect()
}

fn run_solver(
    solver: &Solver,
    input: &[String],
    expected_output: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Running solver for {}-{:02}-{}, level {}: {}",
        solver.year, solver.day, solver.level, solver.level, solver.name
    );

    let input_refs: Vec<&str> = input.iter().map(|s| s.as_str()).collect();
    let start = Instant::now();
    let output = (solver.func)(&input_refs);
    let duration = start.elapsed();

    println!("{}", output);

    if let Some(expected) = expected_output {
        let expected_parts: Vec<&str> = expected.split("\n\n\n").collect();
        let expected = expected_parts.get(solver.level as usize - 1)
            .map(|s| s.trim())
            .unwrap_or("");
        
        if output.trim() == expected {
            println!("SUCCESS!");
        } else {
            println!("FAILED! Expected `{}` but got `{}`", expected, output.trim());
        }
    }

    println!(
        "Solver ran in {}:{:02}:{:02}.{:09}",
        duration.as_secs() / 3600,
        (duration.as_secs() % 3600) / 60,
        duration.as_secs() % 60,
        duration.subsec_nanos()
    );
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_resolve_input_path() {
        let temp = TempDir::new().unwrap();
        let year_dir = temp.path().join("input").join("2024");
        fs::create_dir_all(&year_dir).unwrap();
        
        let input_path = year_dir.join("01.in");
        File::create(&input_path).unwrap().write_all(b"test").unwrap();
        
        let sample_path = year_dir.join("01-sample.in");
        File::create(&sample_path).unwrap().write_all(b"test").unwrap();

        // Temporarily change the current directory to the temp dir
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp.path()).unwrap();

        assert!(resolve_input_path(2024, 1, None).is_ok());
        assert!(resolve_input_path(2024, 1, Some("sample")).is_ok());
        assert!(resolve_input_path(2024, 2, None).is_err());

        // Restore the original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_get_year_and_day() {
        assert!(matches!(get_year_and_day(Some(2015), Some(1)), Ok((2015, 1))));
        assert!(get_year_and_day(Some(2014), Some(1)).is_err());
        assert!(get_year_and_day(Some(2015), Some(26)).is_err());
    }
} 