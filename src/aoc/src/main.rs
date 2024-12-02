use clap::{Parser, Subcommand};

mod prep;
mod fs;
mod aoc_client;
mod run;

extern crate solutions;

use prep::parse_year_or_day;
use run::YearOrDayOrInput;

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
        #[arg(short = 'n', long)]
        dry_run: bool,
    },
    /// Run a solution
    Run {
        first: Option<String>,
        second: Option<String>,
        third: Option<String>,
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
        Commands::Run { first, second, third, level, solver } => {
            let first = first.as_deref().map(YearOrDayOrInput::new).transpose().unwrap();
            let second = second.as_deref().map(YearOrDayOrInput::new).transpose().unwrap();
            let third = third.as_deref().map(YearOrDayOrInput::new).transpose().unwrap();
            let config = run::RunConfig::new(
                first,
                second,
                third,
                level,
                solver,
            );
            if let Err(e) = run::handle(config) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Solvers => {
            if let Err(e) = aoc_core::list_solvers() {
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
        
        // Check that dry_run flag exists with correct options
        let dry_run_arg = args.iter()
            .find(|a| a.get_id().as_str() == "dry_run")
            .expect("dry_run flag should exist");
    
        assert!(
            dry_run_arg.get_short() == Some('n'),
            "dry_run should have short flag '-n'"
        );
        assert!(
            dry_run_arg.get_long() == Some("dry-run"),
            "dry_run should have long flag '--dry-run'"
        );
    }
}
