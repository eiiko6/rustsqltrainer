use axum::{
    Json, Router,
    extract::Path,
    response::IntoResponse,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

use crate::EXERCISES_TOML;
use crate::core::{execute_query, load_exercises_from_str};

#[derive(Serialize, Debug)]
struct Exercise {
    id: i32,
    title: String,
    description: String,
}

#[derive(Deserialize)]
struct SubmitRequest {
    id: i32,
    query: String,
}

#[derive(Serialize)]
struct SubmitResponse {
    correct: bool,
    message: String,
}

pub fn routes() -> Router {
    Router::new()
        .route("/exercises", get(list_exercises))
        .route("/exercise/{id}", get(get_exercise))
        .route("/submit", post(submit))
}

async fn list_exercises() -> impl IntoResponse {
    let exs = load_exercises_from_str(EXERCISES_TOML);
    let list: Vec<Exercise> = exs
        .iter()
        .map(|(id, ex)| Exercise {
            id: *id,
            title: ex.title.clone(),
            description: ex.description.clone(),
        })
        .collect();

    Json(list)
}

async fn get_exercise(Path(id): Path<i32>) -> impl IntoResponse {
    let exs = load_exercises_from_str(EXERCISES_TOML);
    if let Some(ex) = exs.get(&id) {
        Json(Some(Exercise {
            id,
            title: ex.title.clone(),
            description: ex.description.clone(),
        }))
    } else {
        Json(None::<Exercise>)
    }
}

async fn submit(Json(payload): Json<SubmitRequest>) -> impl IntoResponse {
    let exercises = load_exercises_from_str(EXERCISES_TOML);

    if let Some(ex) = exercises.get(&payload.id) {
        match execute_query(&payload.query, false) {
            Ok(result) => {
                if let Ok(expected) = execute_query(&ex.solution, false) {
                    let correct = result == expected;
                    Json(SubmitResponse {
                        correct,
                        message: if correct {
                            "Correct!".into()
                        } else {
                            "Incorrect.".into()
                        },
                    })
                } else {
                    Json(SubmitResponse {
                        correct: false,
                        message: "Error running expected query".into(),
                    })
                }
            }
            Err(e) => Json(SubmitResponse {
                correct: false,
                message: format!("Error: {e}"),
            }),
        }
    } else {
        Json(SubmitResponse {
            correct: false,
            message: "Exercise not found".into(),
        })
    }
}
