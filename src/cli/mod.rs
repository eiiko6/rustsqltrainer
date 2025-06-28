use clap::{Args, Parser, Subcommand};

/// A progam that provide exercises on SQL queries
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,

    /// Activate verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// See where you are at
    Status,

    /// Continue where you were at
    Continue,

    /// Do a specific exercise
    Exercise(ExerciseArgs),

    /// Execute a query, nothing else
    Query(QueryArgs),

    /// Just initialize the db (not required with the other commands)
    Init,

    /// Reset the database, effectively deleting it
    Reset,
}

#[derive(Args, Debug)]
pub struct ExerciseArgs {
    pub id: i32,
}

#[derive(Args, Debug)]
pub struct QueryArgs {
    pub query: String,
}
