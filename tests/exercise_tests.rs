use rsqlt::EXERCISES_TOML;
use rsqlt::core::execute_query;
use rsqlt::core::exercises;
use rsqlt::core::reset_db;
use rsqlt::core::setup_db;

/// Test if exercises can be loaded
#[test]
fn test_exercise_loading() {
    let exercises = exercises::load_exercises_from_str(EXERCISES_TOML);
    assert!(!exercises.is_empty());
}

/// Test if the database can be setup
#[test]
fn test_load_database() {
    let _ = reset_db(true);
    let db = setup_db(true);

    if let Err(e) = &db {
        eprintln!("Database creation failed: {:?}", e);
    }

    assert!(db.is_ok());
}

/// Test if all the exercise solution queries return data
#[test]
fn test_exercise_solutions() {
    let exercises = exercises::load_exercises_from_str(EXERCISES_TOML);
    let _ = reset_db(true);
    setup_db(true).unwrap();
    for (_, val) in exercises {
        let res = execute_query(&val.solution, true);

        if let Err(e) = &res {
            eprintln!("Query failed: {:?}", e);
        }

        assert!(res.is_ok());
        assert!(!res.unwrap().1.is_empty());
    }
}
