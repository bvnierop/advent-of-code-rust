use clap::{Parser, Subcommand};

mod prep;

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
        /// First argument (year or day)
        #[arg(value_parser = prep::parse_year_or_day)]
        first: Option<prep::YearOrDay>,
        
        /// Second argument (year or day)
        #[arg(value_parser = prep::parse_year_or_day)]
        second: Option<prep::YearOrDay>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Prep { first, second } => prep::handle(first, second),
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
        assert_eq!(args.len(), 2, "prep should accept two optional arguments");
    }
}
