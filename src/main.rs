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
    }
}
