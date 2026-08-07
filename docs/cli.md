# EIR CLI

The EIR CLI is the primary command-line interface for working with Entity Identifier Resolver databases.

It is built on top of `eir-core` and provides tools for building, inspecting, and searching `.eir` databases. While primarily intended for development and debugging, it is also useful for validating datasets and experimenting with search behavior.

## Installation

Run directly from the workspace:

```bash
cargo run -p eir-cli -- --help
```

or using the workspace alias:

```bash
cargo eir --help
```

---

# Commands

## Build

Build an `.eir` database from one or more input files.

```bash
cargo eir build data/entities.json database.eir
```

This command:

- Reads entity documents
- Builds all search indexes
- Serializes the database into a compact `.eir` file

---

## Search

Search an existing database.

```bash
cargo eir search database.eir "FizzBerry"
```

Example output:

```
Search: FizzBerry

FizzBerry Spark
score=1.00

Signals
-------
✓ ExactAlias
✓ PrefixAlias
✓ Token
```

Search displays:

- matching entities
- relevance score
- signals contributing to the score
- optional explanation of why the result matched

---

## Inspect

Inspect the contents of an EIR database.

```bash
cargo eir inspect database.eir
```

Displays information such as:

- entity count
- aliases
- tags
- relationships
- registered properties
- registered sources
- index statistics

---

# Development Workflow

A common workflow is:

```text
 Entity Documents
        │
        ▼
 cargo eir build
        │
        ▼
 database.eir
        │
        ├─────────────┐
        ▼             ▼
cargo eir search   cargo eir inspect
```

---

# Why use the CLI?

The CLI makes it easy to:

- verify datasets before shipping
- test search quality
- debug scoring signals
- inspect generated indexes
- benchmark search performance
- automate builds in CI

Since it uses the same library as applications embedding EIR, results are consistent between development and production.
