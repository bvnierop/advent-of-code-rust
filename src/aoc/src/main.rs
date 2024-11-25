use clap::{Parser, Subcommand};
use time::{OffsetDateTime, Month};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Prepare environment for solving a problem
    Prep {
        /// Year of the problem to prepare
        #[arg(value_parser = clap::value_parser!(u16).range(2015..))]
        year: Option<u16>,
        
        /// Day of the problem to prepare (1-25)
        #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
        day: Option<u8>,
    },
}

fn get_current_advent() -> (u16, u8) {
    let now = OffsetDateTime::now_utc();
    let year = now.year() as u16;
    
    // Only return current day if we're in December and the day is 1-25
    let day = if now.month() == Month::December && (1..=25).contains(&now.day()) {
        now.day() as u8
    } else {
        1
    };
    
    (year, day)
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Prep { year, day } => {
            let (current_year, current_day) = get_current_advent();
            let year = year.unwrap_or(current_year);
            let day = day.unwrap_or(current_day);
            
            println!("Preparing environment for year {} day {}...", year, day);
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
        
        assert!(
            prep_cmd.get_arguments().any(|a| a.get_id().as_str() == "year"),
            "prep should accept year argument"
        );
        assert!(
            prep_cmd.get_arguments().any(|a| a.get_id().as_str() == "day"),
            "prep should accept day argument"
        );
    }
}
