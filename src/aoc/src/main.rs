use clap::{Parser, Subcommand};

mod prep;
mod fs;
mod aoc_client;
mod run;
mod solver;
mod types;

// Import solutions crate to ensure solvers are registered
extern crate solutions;
solutions::inventory::collect!();

use prep::parse_year_or_day;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Prepare the environment for solving a problem
    Prep {
        first: Option<String>,
        second: Option<String>,
        #[arg(long)]
        dry_run: bool,
    },
    /// Run a solution
    Run {
        year: Option<u16>,
        day: Option<u8>,
        input_file: Option<String>,
        #[arg(long)]
        level: Option<u8>,
        #[arg(long)]
        solver: Option<String>,
    },
    /// List all available solvers
    Solvers,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Prep { first, second, dry_run } => {
            let first = first.as_deref().map(parse_year_or_day).transpose().unwrap();
            let second = second.as_deref().map(parse_year_or_day).transpose().unwrap();
            prep::handle(first, second, dry_run);
        }
        Commands::Run { year, day, input_file, level, solver } => {
            let config = run::RunConfig {
                year,
                day,
                input_file,
                level,
                solver,
            };
            if let Err(e) = run::handle(config) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Solvers => {
            if let Err(e) = solver::list_solvers() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert()
    }

    #[test]
    fn verify_prep_command_exists() {
        let cmd = Cli::command();
        assert!(
            cmd.get_subcommands().any(|c| c.get_name() == "prep"),
            "prep command should exist"
        );
    }

    #[test]
    fn verify_prep_args() {
        let cmd = Cli::command();
        let prep_cmd = cmd.get_subcommands()
            .find(|c| c.get_name() == "prep")
            .expect("prep command should exist");
        
        let args: Vec<_> = prep_cmd.get_arguments().collect();
        
        assert_eq!(args.len(), 3, "prep should accept two optional arguments and a dry-run flag");
        
        assert!(
            args.iter().any(|a| a.get_id().as_str() == "dry_run"),
            "prep should have a dry-run flag"
        );
    }
}
