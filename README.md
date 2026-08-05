# Entity-Identifier-Resolver (EIR)

## Note
It is in its earliest state; many features are missing or not implementet yes.

# ⚠️Warning⚠️
EIR is *NOT* encrypted.
Do *NOT* use it to store sensitive data!
It is optimized for search.

# EIR CLI Quick Start

The `eir` CLI is the developer tool for the **Entity Identifier Resolver**. It is used to build databases, inspect entities, manage indexes, and test search.

## Commands

```text
build        Build an EIR database
stats        Show database statistics
inspect      Inspect an entity
search       Search entities
index        Manage indexes
generate     Generate test datasets
completions  Generate shell completions
```

## Build a database

```bash
cargo eir build \
  --input entities.json \
  --output database.eir
```

## Inspect an entity

```bash
cargo eir inspect database.eir --entity 1
```

Example:

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
  food

Relationships:
  MadeBy -> Aurora Foods
  InstanceOf -> Drink
  LocatedIn -> Denmark

Sources:
  Open Food Facts
```

Verbose mode shows internal IDs:

```bash
cargo eir inspect database.eir --entity 1 --verbose
```

## Search entities

```bash
cargo eir search database.eir fizzberry
```

Output:

```text
Search: fizzberry

FizzBerry Spark score=1.00 via=ExactAlias
```

Search supports:

* Exact aliases
* Prefix matching
* Fuzzy matching
* Token search
* Tags
* Properties
* Sources
* Relationships

Example:

```bash
cargo eir search database.eir drink
```

Output:

```text
Search: drink

Drink                  score=1.00 via=ExactAlias
Crystal Spring Water   score=0.70 via=Relationship
FizzBerry Spark        score=0.70 via=Relationship
```

## Development workflow

```bash
# Generate test data
cargo eir generate

# Build database
cargo eir build --input entities.json --output database.eir

# Inspect data
cargo eir inspect database.eir --entity 1

# Test search
cargo eir search database.eir fizzberry
```

EIR is designed for fast local entity resolution using aliases, indexes, metadata, and relationships.

*Contains AI-assisted code
All art and icons are made by Humans*
