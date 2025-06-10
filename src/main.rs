use core::{execute_query, reset_db, setup_db};

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
        cli::Action::Exercise(ex_args) => {
            let exercises = core::load_exercises("./exercises.toml");
            if let Some(ex) = exercises.get(&ex_args.id) {
                println!("Title: {}", ex.title);
                println!("Description: {}", ex.description);

                // Setup DB
                if let Err(e) = setup_db(args.verbose) {
                    eprintln!("Failed to initialize database: {e}");
                    return;
                }

                // Execute solution query to get the expected results
                let expected = match execute_query(&ex.solution, args.verbose) {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("Failed to execute solution query: {e}");
                        return;
                    }
                };

                // Prompt player for input query
                println!("Enter your SQL query:");

                let mut input_query = String::new();
                std::io::stdin()
                    .read_line(&mut input_query)
                    .expect("Failed to read input");
                let input_query = input_query.trim();

                // Execute player's query
                let player_result = match execute_query(input_query, args.verbose) {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("Failed to execute your query: {e}");
                        return;
                    }
                };

                // Compare player's result with expected result
                if player_result == expected {
                    println!("Correct!");
                } else {
                    println!("Incorrect. Your query result did not match the expected output.");
                    println!("Expected:");
                    for (i, row) in expected.iter().enumerate() {
                        println!("Row {}:", i);
                        for (col, val) in row.iter() {
                            println!("  {}: {}", col, val);
                        }
                    }
                    println!("Got:");
                    for (i, row) in player_result.iter().enumerate() {
                        println!("Row {}:", i);
                        for (col, val) in row.iter() {
                            println!("  {}: {}", col, val);
                        }
                    }
                }
            } else {
                eprintln!("Exercise with id {} not found.", ex_args.id);
            }
        }
        cli::Action::Init => {
            if let Err(e) = setup_db(args.verbose) {
                eprintln!("Failed to initialize database: {e}");
                return;
            }

            println!("Initialized database.");
        }
        cli::Action::Reset => {
            if let Err(e) = reset_db(args.verbose) {
                eprintln!("Failed to reset database: {e}");
            }
        }
    }
}
