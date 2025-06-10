use colored::Colorize;
use core::{execute_query, get_db_path, reset_db, setup_db};
use rustyline::DefaultEditor;

use clap::Parser;
mod cli;
mod core;

fn display_results(results: (Vec<String>, Vec<Vec<String>>)) {
    let headers = &results.0;
    let rows = &results.1;

    let cols = headers.len();
    let mut max_widths = vec![0; cols];

    for (i, header) in headers.iter().enumerate() {
        max_widths[i] = header.len();
    }

    for row in rows {
        for (i, val) in row.iter().enumerate() {
            if val.len() > max_widths[i] {
                max_widths[i] = val.len();
            }
        }
    }

    // Print header with padding
    for (i, header) in headers.iter().enumerate() {
        print!("{:<width$} ", header.blue(), width = max_widths[i]);
    }
    println!();

    // Print rows with padding and blue color
    for row in rows {
        for (i, val) in row.iter().enumerate() {
            print!("{:<width$} ", val, width = max_widths[i]);
        }
        println!();
    }
}

fn main() {
    let args = cli::Cli::parse();

    let mut history_path = get_db_path(args.verbose);
    history_path.set_file_name("history.txt");

    match args.action {
        cli::Action::Status => {
            println!("{}", "Not yet implemented.".yellow());
        }
        cli::Action::Continue => {
            println!("{}", "Not yet implemented.".yellow());
        }
        cli::Action::Exercise(ex_args) => {
            let exercises = core::load_exercises("./exercises.toml");
            if let Some(ex) = exercises.get(&ex_args.id) {
                println!("Title: {}", ex.title.bold().green());
                println!("Description: {}", ex.description);

                // Setup DB
                if let Err(e) = setup_db(args.verbose) {
                    eprintln!("{}", format!("Failed to initialize database: {e}").red());
                    return;
                }

                // Execute solution query to get the expected results
                let expected = match execute_query(&ex.solution, args.verbose) {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("{}", format!("Failed to execute solution query: {e}").red());
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
                        eprintln!("{}", format!("Failed to read input: {e}").red());
                        return;
                    }
                };

                // Save history back to file after the input
                if let Err(e) = rl.save_history(&history_path) {
                    eprintln!("{}", format!("Failed to save history: {e}").yellow());
                }

                // Execute player's query
                let player_result = match execute_query(&input_query, args.verbose) {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("{}", format!("Failed to execute your query: {e}").red());
                        return;
                    }
                };

                // Compare player's result with expected result
                if player_result == expected {
                    println!("{}", "Correct!".bold().green());
                } else {
                    println!(
                        "{}",
                        "Incorrect. Your query result did not match the expected output."
                            .bold()
                            .red()
                    );

                    println!("{}", "Expected:".underline().yellow());
                    display_results(expected);

                    println!("{}", "Got:".underline().yellow());
                    display_results(player_result);
                }
            } else {
                eprintln!(
                    "{}",
                    format!("Exercise with id {} not found.", ex_args.id).red()
                );
            }
        }
        cli::Action::Query(query_args) => {
            let result = match execute_query(&query_args.query, args.verbose) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("{}", format!("Failed to execute your query: {e}").red());
                    return;
                }
            };

            display_results(result);
        }
        cli::Action::Init => {
            if let Err(e) = setup_db(args.verbose) {
                eprintln!("{}", format!("Failed to initialize database: {e}").red());
                return;
            }

            println!("{}", "Initialized database.".bold().green());
        }
        cli::Action::Reset => {
            if let Err(e) = reset_db(args.verbose) {
                eprintln!("{}", format!("Failed to reset database: {e}").red());
            }
        }
    }
}
