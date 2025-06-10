# Rust SQL Trainer (rsqlt)

A program that provides exercises on SQL queries.

## Usage

```bash
rsqlt [OPTIONS] <SUBCOMMAND>
```

Note that this can only be used in the repository as it needs the toml files.
The database is stored in your cache directory.

This uses [rustyline](https://crates.io/crates/rustyline/), so it keeps a history of you previous requests that you can navigate just like in a shell.

## Options

* `-v`, `--verbose`
  Activate verbose mode.

## Subcommands

### status (not yet implemented)

See where you are at.

```bash
rsqlt status
```

### continue (not yet implemented)

Continue where you were at.

```bash
rsqlt continue
```

### exercise

Do a specific exercise by its ID.

```bash
rsqlt exercise <id>
```

### query

Execute a raw SQL query directly, nothing else.

```bash
rsqlt query "<SQL_QUERY>"
```

### init

Initialize the database (not required with other commands).

```bash
rsqlt init
```

### reset

Reset the database, effectively deleting it.

```bash
rsqlt reset
```
