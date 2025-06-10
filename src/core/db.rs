use dirs::cache_dir;
use rusqlite::Connection;
use rusqlite::Result;
use serde::Deserialize;
use serde_rusqlite::{columns_from_statement, from_row_with_columns};
use std::collections::HashMap;
use std::fs;
use std::fs::read_to_string;
use std::path::PathBuf;

const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
const DB_NAME: &str = "sport_rental.sqlite";

#[derive(Deserialize)]
struct Db {
    schema: SchemaSection,
    inserts: InsertsSection,
}

#[derive(Deserialize)]
struct SchemaSection {
    data: String,
}

#[derive(Deserialize)]
struct InsertsSection {
    data: String,
}

fn verbose_println(verbose: bool, msg: &str) {
    if verbose {
        println!("{}", msg);
    }
}

pub fn get_db_path(verbose: bool) -> PathBuf {
    let mut path = cache_dir().expect("Could not determine cache directory");
    verbose_println(verbose, &format!("Cache directory: {:?}", path));
    path.push(CRATE_NAME);
    fs::create_dir_all(&path).expect("Failed to create app cache dir");
    path.push(DB_NAME);
    verbose_println(verbose, &format!("Database path: {:?}", path));
    path
}

fn load_schema() -> (String, String) {
    let content = read_to_string("db.toml").expect("Failed to read db.toml");
    let parsed: Db = toml::from_str(&content).expect("Failed to parse db.toml");
    (parsed.schema.data, parsed.inserts.data)
}

pub fn setup_db(verbose: bool) -> Result<()> {
    let path = get_db_path(verbose);
    verbose_println(verbose, &format!("Checking if DB exists at {:?}", path));
    if path.exists() {
        verbose_println(verbose, "DB already exists. Skipping creation.");
        return Ok(());
    }

    verbose_println(verbose, "DB does not exist, creating new database...");
    let conn = Connection::open(&path)?;
    verbose_println(verbose, "Database connection opened.");

    let (schema, inserts) = load_schema();

    conn.execute_batch(&schema)?;
    verbose_println(verbose, "Tables created successfully from TOML schema.");

    conn.execute_batch(&inserts)?;

    Ok(())
}

pub fn reset_db(verbose: bool) -> Result<()> {
    let path = get_db_path(verbose);
    verbose_println(verbose, &format!("Resetting DB at {:?}", path));
    if path.exists() {
        let _ = fs::remove_file(&path);
        verbose_println(verbose, "Database file removed.");
    } else {
        verbose_println(verbose, "Database file does not exist.");
    }
    Ok(())
}

/// Executes a raw SQL query and returns results as a Vec of HashMaps
pub fn execute_query(query: &str, verbose: bool) -> Result<Vec<HashMap<String, String>>> {
    verbose_println(verbose, &format!("Executing query: {}", query));

    let conn = Connection::open_with_flags(
        get_db_path(verbose),
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
    )?;
    verbose_println(verbose, "Database connection opened for query.");

    let mut stmt = conn.prepare(&query.replace("select", "SELECT"))?; // Avoid the "non-read-only" error
    verbose_println(verbose, "Query prepared.");

    let columns = columns_from_statement(&stmt);
    verbose_println(verbose, &format!("Columns extracted: {:?}", columns));

    let rows = stmt.query_map([], |row| {
        from_row_with_columns::<HashMap<String, String>>(row, &columns)
            .map_err(|_| rusqlite::Error::InvalidQuery)
    })?;

    let result = rows.collect::<Result<Vec<_>, _>>()?;
    verbose_println(
        verbose,
        &format!(
            "Query executed successfully, {} rows fetched.",
            result.len()
        ),
    );

    Ok(result)
}
