# Rust SQL Trainer (rsqlt)

A program that provides exercises on SQL queries.  
rsqlt is fully cross platform, and does not require any setup.

## Usage

```bash
rsqlt [OPTIONS] <SUBCOMMAND>
```

The database is stored in your cache directory.

This uses [rustyline](https://crates.io/crates/rustyline/), so it keeps a history of you previous requests that you can navigate just like in a shell.

## Options

* `-v`, `--verbose`
  Activate verbose mode.

## Subcommands

### status (not yet implemented)

See where you are at.

```bash
rsqlt-cli status
```

### continue (not yet implemented)

Continue where you were at.

```bash
rsqlt-cli continue
```

### exercise

Do a specific exercise by its ID.

```bash
rsqlt-cli exercise <id>
```

### query

Execute a raw SQL query directly, nothing else.

```bash
rsqlt-cli query "<SQL_QUERY>"
```

### init

Initialize the database (not required with other commands).

```bash
rsqlt-cli init
```

### reset

Reset the database, effectively deleting it.

```bash
rsqlt-cli reset
```

### web server

Launch the web server and open it in your browser.

```bash
rsqlt-web
```
