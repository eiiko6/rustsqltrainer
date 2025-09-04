# Rust SQL Trainer (rsqlt)

**rsqlt** is a cross-platform program for learning and practicing SQL queries through interactive exercises.

## Features

* Interactive SQL exercises with instant feedback.
* Improved exercise navigation via CLI or web.
* Web frontend built with Vue.js, embedded into a single Rust binary for now.
* Fully cross-platform: Linux, Windows, macOS.
* Persistent SQL query history in CLI mode (powered by [rustyline](https://crates.io/crates/rustyline)).

## Installation

Just compile the Rust project, or download a prebuilt binary. No database setup required — the database is automatically stored in your system’s cache directory.

## CLI Usage

```bash
rsqlt [OPTIONS] <SUBCOMMAND>
```

### Options

* `-v`, `--verbose` — Enable verbose mode.

### Subcommands

#### `status` (planned)

Check your current progress.

```bash
rsqlt-cli status
```

#### `continue` (planned)

Continue from your last unfinished exercise.

```bash
rsqlt-cli continue
```

#### `exercise`

Attempt a specific exercise by its ID.

```bash
rsqlt-cli exercise <id>
```

#### `query`

Execute a raw SQL query directly, without exercises.

```bash
rsqlt-cli query "<SQL_QUERY>"
```

#### `init`

Initialize the database (optional, other commands do this automatically).

```bash
rsqlt-cli init
```

#### `reset`

Reset the database, deleting all progress.

```bash
rsqlt-cli reset
```

## Web Usage

Launch the **embedded web server** and open the interactive Vue frontend in your browser (`locahost:3000`).  
Works on any platform.

```bash
rsqlt-web
```

> ⚠️ **Note:** On the `rsqlt-web` binary, the web frontend communicates with the backend via local ports on your machine.
> This is inelegant and is only meant as a shortcut to let users run the app easily without needing to configure a separate web server.
> This means it is not a ready-to-use app, as this project is (eventually) intended to have its frontend and backend split.

### Development Notes

* Frontend is in `frontend/` and uses Vue.js + Vite.
* For now static assets are embedded in the Rust binary via [rust-embed](https://crates.io/crates/rust-embed) to offer a convenient local web app.
