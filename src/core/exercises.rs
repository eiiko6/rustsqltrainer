use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Exercise {
    pub title: String,
    pub description: String,
    pub solution: String,
}

#[derive(Debug, Deserialize)]
struct ExercisesFile {
    exercises: Vec<Exercise>,
}

pub fn load_exercises<P: AsRef<Path>>(path: P) -> HashMap<i32, Exercise> {
    let content = fs::read_to_string(path).expect("Failed to read the TOML file");
    let exercises_file: ExercisesFile = toml::from_str(&content).expect("Failed to parse TOML");

    exercises_file
        .exercises
        .into_iter()
        .enumerate()
        .map(|(i, ex)| (i as i32 + 1, ex))
        .collect()
}
