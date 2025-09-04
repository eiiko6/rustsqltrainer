use axum::{
    extract::{Form, Path},
    response::Html,
};
use serde::Deserialize;

use crate::EXERCISES_TOML;
use crate::core::{execute_query, load_exercises_from_str};

pub async fn home() -> Html<String> {
    let ex = load_exercises_from_str(EXERCISES_TOML);
    let list = ex
        .iter()
        .map(|(id, ex)| format!("<li><a href=\"/exercise/{}\">{}</a></li>", id, ex.title))
        .collect::<Vec<_>>()
        .join("\n");

    Html(format!("<h1>Exercises</h1><ul>{list}</ul>"))
}

pub async fn exercise(Path(id): Path<String>) -> Html<String> {
    let exs = load_exercises_from_str(EXERCISES_TOML);
    let parsed_id: i32 = match id.parse() {
        Ok(n) => n,
        Err(_) => return Html("Invalid ID".into()),
    };

    if let Some(ex) = exs.get(&parsed_id) {
        Html(format!(
            "<h1>{}</h1><p>{}</p><form action=\"/submit\" method=\"post\">
                <input type=\"hidden\" name=\"id\" value=\"{}\"/>
                <textarea name=\"query\"></textarea><br/>
                <button type=\"submit\">Submit</button>
             </form>",
            ex.title, ex.description, id
        ))
    } else {
        Html("<p>Exercise not found</p>".into())
    }
}

#[derive(Deserialize)]
pub struct QueryForm {
    id: String,
    query: String,
}

pub async fn submit(Form(form): Form<QueryForm>) -> Html<String> {
    let exercises = load_exercises_from_str(EXERCISES_TOML);
    let parsed_id: i32 = match form.id.parse() {
        Ok(n) => n,
        Err(_) => return Html("Invalid exercise ID".into()),
    };

    if let Some(ex) = exercises.get(&parsed_id) {
        match execute_query(&form.query, false) {
            Ok(result) => {
                if let Ok(expected) = execute_query(&ex.solution, false) {
                    let correct = result == expected;
                    Html(format!(
                        "<p>{}</p><a href=\"/exercise/{}\">Back</a>",
                        if correct {
                            "<strong style=\"color: green\">Correct!</strong>"
                        } else {
                            "<strong style=\"color: red\">Incorrect!</strong>"
                        },
                        form.id
                    ))
                } else {
                    Html("Error running expected query".into())
                }
            }
            Err(e) => Html(format!("<p>Error: {}</p>", e)),
        }
    } else {
        Html("Exercise not found".into())
    }
}
