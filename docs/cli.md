# EIR CLI Documentation

## Overview

`eir` is the command-line interface for the **Entity Identifier Resolver (EIR)**.

The CLI provides tools for building entity databases, inspecting stored entities, generating datasets, managing indexes, and testing the EIR search engine.

EIR is designed around local entity resolution: resolving names, aliases, tags, attributes, sources, and relationships into known entities.

---

# Installation

During development, the CLI can be run through Cargo:

```bash
cargo run -p eir-cli -- <COMMAND>
```

A shorter development command is available:

```bash
cargo eir <COMMAND>
```

This uses Cargo's custom command runner to execute the `eir-cli` binary.

After installation, the binary can also be run directly:

```bash
eir <COMMAND>
```

---

# Commands

Current commands:

```text
build        Build datasets
stats        Show database statistics
inspect      Inspect an entity
index        Build or manage indexes
search       Search entities
generate     Generate datasets
completions  Generate shell completions
help         Print help information
```

---

# Build

Build an EIR database from an entity dataset.

## Usage

```bash
eir build --input <DATASET> --output <DATABASE>
```

Example:

```bash
cargo eir build \
  --input fixtures/entities.json \
  --output data/database.eir
```

This converts the source entity data into the optimized EIR database format.

The database contains:

- Entities
- Aliases
- Tags
- Attributes
- Relationships
- Sources
- Search indexes

---

# Stats

Display database statistics.

## Usage

```bash
eir stats <DATABASE>
```

Example:

```bash
cargo eir stats database.eir
```

Example output:

```text
Entities: 30
Tags: 42
Attributes: 8
Sources: 3
Relationships: 25
```

---

# Inspect

Inspect a single entity.

## Usage

```bash
eir inspect <DATABASE> --entity <ID>
```

Example:

```bash
cargo eir inspect database.eir --entity 1
```

Output:

```text
Entity: 1

Names:
  FizzBerry Spark
  Berry Spark
  FizzBerry

Tags:
  sparkling
  berry
  drink
  sweet
  food

Attributes:
  volume

Relationships:
  MadeBy -> Aurora Foods
  InstanceOf -> Drink
  LocatedIn -> Denmark

Sources:
  Open Food Facts
```

---

## Verbose Inspect

Verbose mode displays internal IDs.

Usage:

```bash
eir inspect database.eir --entity 1 --verbose
```

Example:

```text
Tags:
  sparkling (9)
  berry (10)
  drink (11)

Relationships:
  MadeBy -> Aurora Foods (1000)
  InstanceOf -> Drink (2000)
  LocatedIn -> Denmark (3000)
```

This mode is useful when debugging indexes and relationships.

---

# Search

Search entities using the EIR resolver.

## Usage

```bash
eir search <DATABASE> <QUERY>
```

Example:

```bash
cargo eir search database.eir fizz
```

Output:

```text
Search: fizz

FizzBerry Spark score=0.80 via=PrefixAlias
```

---

# Search Types

EIR combines multiple search strategies.

## Exact Alias

Highest confidence match.

Example:

```bash
eir search database.eir fizzberry
```

Result:

```text
FizzBerry Spark score=1.00 via=ExactAlias
```

---

## Prefix Search

Matches names beginning with the query.

Example:

```bash
eir search database.eir aurora
```

Result:

```text
Aurora Foods          score=1.00 via=ExactAlias
Aurora Tomato Soup    score=0.80 via=PrefixAlias
Aurora Wholegrain Bread score=0.80 via=PrefixAlias
```

---

## Fuzzy Search

Handles spelling mistakes.

Example:

```bash
eir search database.eir fizzbery
```

Result:

```text
FizzBerry Spark score=0.60 via=FuzzyAlias
```

---

## Token Search

Searches individual words.

Example:

```bash
eir search database.eir water
```

Result:

```text
Crystal Spring Water
PureSpring Sparkling Water
PureSpring Mineral Water
```

---

## Tag Search

Matches entity tags.

Example:

```bash
eir search database.eir drink
```

Possible matches:

```text
PureSpring Mineral Water score=0.40 via=Tag
```

---

## Relationship Search

Finds entities connected through relationships.

Example:

```bash
eir search database.eir drink
```

Result:

```text
Drink                  score=1.00 via=ExactAlias
Crystal Spring Water   score=0.70 via=Relationship
FizzBerry Spark        score=0.70 via=Relationship
```

Because:

```text
FizzBerry Spark
    |
    └── InstanceOf → Drink
```

---

# Index Management

Manage search indexes.

## Usage

```bash
eir index <COMMAND>
```

Available commands:

```text
build
stats
```

Example:

```bash
eir index build --input database.eir --output indexes/
```

Indexes include:

- Alias index
- Prefix trie
- BK-tree fuzzy index
- Token inverted index
- Tag posting lists
- Property posting lists
- Source posting lists
- Relationship indexes

---

# Generate

Generate test datasets.

Example:

```bash
eir generate
```

Useful for:

- Development
- Benchmarks
- Testing search behaviour

---

# Shell Completions

Generate shell completion scripts.

Supported shells:

- Bash
- Zsh
- Fish
- PowerShell
- Elvish

Example:

```bash
eir completions PowerShell
```

---

# Search Ranking

EIR assigns scores based on match confidence.

Current scoring:

| Match Type   | Score |
| ------------ | ----: |
| Exact Alias  |  1.00 |
| Prefix Alias |  0.80 |
| Relationship |  0.70 |
| Fuzzy Alias  |  0.60 |
| Token        |  0.50 |
| Tag          |  0.40 |
| Property     |  0.30 |
| Source       |  0.20 |

Higher scores appear first.

---

# Development Workflow

Typical workflow:

## 1. Generate data

```bash
cargo eir generate
```

## 2. Build database

```bash
cargo eir build \
  --input fixtures/entities.json \
  --output database.eir
```

## 3. Inspect entities

```bash
cargo eir inspect database.eir --entity 1
```

## 4. Test search

```bash
cargo eir search database.eir fizzberry
```

## 5. Debug indexes

Use:

```bash
--verbose
```

with inspection commands.

---

# Future Improvements

Potential future CLI features:

- Interactive search mode
- JSON output mode
- Export resolved entities
- Index benchmarking
- Search explanation mode
- Relationship graph inspection
- Database migration tools
- Import/export commands

---

# Summary

The EIR CLI provides a complete developer interface for managing the Entity Identifier Resolver database.

It allows developers to:

- Build entity databases
- Inspect entity structures
- Test search quality
- Debug indexes
- Explore relationships
- Benchmark resolver behaviour

The CLI is the primary development tool for building and maintaining EIR datasets and search infrastructure.
