use core::{execute_query, get_db_path, reset_db, setup_db};
use rustyline::DefaultEditor;

use clap::Parser;
mod cli;
mod core;

fn main() {
    let args = cli::Cli::parse();

    let mut history_path = get_db_path(args.verbose);
    history_path.set_file_name("history.txt");

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
                let mut rl = DefaultEditor::new().unwrap();
                if rl.load_history(&history_path).is_err() {
                    // No previous history file found, ignore error
                }

                let input_query = match rl.readline("Enter your SQL query: ") {
                    Ok(line) => {
                        let trimmed = line.trim().to_string();
                        if !trimmed.is_empty() {
                            rl.add_history_entry(trimmed.as_str()).unwrap();
                        }
                        trimmed
                    }
                    Err(e) => {
                        eprintln!("Failed to read input: {e}");
                        return;
                    }
                };

                // Save history back to file after the input
                if let Err(e) = rl.save_history(&history_path) {
                    eprintln!("Failed to save history: {e}");
                }

                // Execute player's query
                let player_result = match execute_query(&input_query, args.verbose) {
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
        cli::Action::Query(query_args) => {
            let result = match execute_query(&query_args.query, args.verbose) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("Failed to execute your query: {e}");
                    return;
                }
            };

            for (i, row) in result.iter().enumerate() {
                println!("Row {}:", i);
                for (col, val) in row.iter() {
                    println!("  {}: {}", col, val);
                }
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
