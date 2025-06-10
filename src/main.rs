use core::{execute_query, setup_db};

use clap::Parser;
mod cli;
mod core;

fn main() {
    let args = cli::Cli::parse();

    match args.action {
        cli::Action::Status => {
            println!("Not yet implemented.");
        }
        cli::Action::Continue => {
            println!("Not yet implemented.");
        }
        cli::Action::Exercise(args) => {
            let exercises = core::load_exercises("./exercises.toml");
            if let Some(ex) = exercises.get(&args.id) {
                println!("Title: {}", ex.title);
                println!("Description: {}", ex.description);
                println!("Solution: {}", ex.solution);
            } else {
                eprintln!("Exercise with id {} not found.", args.id);
            }
        }
        cli::Action::Init => {
            if let Err(e) = setup_db(args.verbose) {
                eprintln!("Failed to initialize database: {e}");
                return;
            }

            let result = execute_query("SELECT * FROM CLIENT;", args.verbose);

            if let Err(e) = result {
                eprintln!("Failed to execute query: {e}");
            } else {
                for (i, row) in result.unwrap().iter().enumerate() {
                    println!("Row {}:", i);
                    for (col, val) in row.iter() {
                        println!("  {}: {}", col, val);
                    }
                }
            }

            println!("Initialized database.");
        }
    }
}

