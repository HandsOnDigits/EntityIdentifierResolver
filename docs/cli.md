# EIR CLI Documentation

## Overview

`eir` is the command-line interface for the Entity Identifier Resolver (EIR).

The CLI provides tools for building, inspecting, searching, modifying, and debugging local EIR databases.

EIR is designed around local entity resolution: resolving names, aliases, tokens, tags, attributes, sources, and relationships into known entities.

The CLI is also the primary development and testing interface for EIR. It makes it possible to build a database, inspect its contents, test search behavior, insert and remove entities, and verify that changes persist correctly.

---

# Installation

During development, the CLI can be run through Cargo:

```powershell
cargo run -p eir-cli -- <COMMAND>
```

A shorter development command is also available:

```powershell
cargo eir <COMMAND>
```

After installation, the binary can be run directly:

```powershell
eir <COMMAND>
```

---

# Commands

Current commands include:

```text
build
stats
inspect
insert
remove
search
index
generate
completions
help
```

Use:

```powershell
eir help
```

or:

```powershell
eir <COMMAND> --help
```

to see command-specific options.

---

# Database Lifecycle

An EIR database normally follows this lifecycle:

```text
Dataset
   │
   ▼
Build
   │
   ▼
Database
   │
   ├── Inspect
   ├── Search
   ├── Insert
   └── Remove
          │
          ▼
      Rebuild indexes
```

Database mutations update the stored entity collection and rebuild the search indexes so that subsequent searches operate on the current database contents.

---

# Build

Build an EIR database from an entity dataset.

## Usage

```powershell
eir build --input <DATASET> --output <DATABASE>
```

Example:

```powershell
cargo eir build `
  --input fixtures/entities.json `
  --output data/entities.eir
```

The build process converts the source entity data into the EIR database format.

The resulting database contains:

- Entities
- Aliases
- Tags
- Attributes
- Relationships
- Sources
- Search indexes
- Internal registries

The database is serialized to disk and can subsequently be opened by the other CLI commands.

---

# Stats

Display database statistics.

## Usage

```powershell
eir stats <DATABASE>
```

Example:

```powershell
cargo eir stats data/entities.eir
```

Example output:

```text
# Database Statistics

Entities:           46
Tags:               43
Sources:            2
attributes:         15
Relationship Types: 5

## Indexes

Aliases:     89
Trie:        89
BK-Tree:     92
Tokens:      74
Tags:        43
Sources:     2
Relationships: 39
```

Statistics are useful for verifying that a database was built correctly and for checking the effect of database mutations.

For example, inserting an entity can increase the entity count and index sizes, while removing it should remove the entity from the searchable index structures.

---

# Inspect

Inspect a single entity by ID.

## Usage

```powershell
eir inspect <DATABASE> --entity <ID>
```

Example:

```powershell
cargo eir inspect data/entities.eir --entity 9100
```

Example output:

```text
Entity: 9100

Names:
Test Berry

Tags:
test

Attributes:

Relationships:

Sources:
test source
```

Inspection displays the entity document stored in the database, including:

- Entity ID
- Names and aliases
- Tags
- Attributes
- Relationships
- Sources

If the entity does not exist, the command exits with an error:

```text
Error: Entity not found
```

---

# Search

Search entities using the EIR query and search pipeline.

## Usage

```powershell
eir search <DATABASE> <QUERY>
```

Example:

```powershell
cargo eir search data/entities.eir "FizzBerry"
```

Example:

```text
Search: FizzBerry

FizzBerry Spark score=1.00
Signals:
Token
PrefixAlias
ExactAlias
FuzzyAlias
Why:
ExactAlias { alias: "fizzberry" }
PrefixAlias { alias: "fizzberry" }
FuzzyAlias { alias: "fizzberry" }
Token { token: "fizzberry" }

FizzBerry Energy Blast score=0.78
Signals:
Token
PrefixAlias
Why:
PrefixAlias { alias: "fizzberry" }
Token { token: "fizzberry" }
```

Search results combine multiple signals and are ranked by the EIR ranker.

A result may contain several signals simultaneously. This allows strong matches to accumulate evidence rather than relying on a single search strategy.

---

# Search Types

EIR currently combines several search strategies.

## Exact Alias

Matches an entity alias exactly after normalization.

Example:

```powershell
eir search data/entities.eir "FizzBerry"
```

An exact alias can produce a high-confidence result:

```text
FizzBerry Spark score=1.00
```

The explanation identifies the exact alias signal.

---

## Prefix Alias

Matches aliases beginning with the query.

Example:

```powershell
eir search data/entities.eir "Fizz"
```

This can return entities such as:

```text
FizzBerry Spark
FizzBerry Spark Zero
FizzBerry Energy Blast
```

Prefix matches are useful for autocomplete-style queries and incomplete names.

---

## Fuzzy Alias

The fuzzy alias index uses the BK-tree to find aliases that are close to the query.

Example:

```powershell
eir search data/entities.eir "Fizzbery"
```

This allows minor spelling errors to still resolve to known entities.

---

## Token Search

Aliases are normalized and split into searchable tokens.

For example:

```text
FizzBerry Spark Zero
```

produces searchable terms including:

```text
fizzberry
spark
zero
```

A query for an individual token can therefore match multiple entities.

Example:

```powershell
eir search data/entities.eir "water"
```

may return:

```text
Crystal Spring Water
PureSpring Sparkling Water
PureSpring Mineral Water
```

---

## Tag Search

Entities can be associated with tags.

A tag query can identify entities belonging to that tag.

Example:

```powershell
eir search data/entities.eir "drink"
```

A matching entity can contain:

```text
Signals:
Tag
```

Tag matches are ranked separately from alias and token matches.

---

## Attribute Search

Attributes can be searched by key, value, or key/value pair.

For example, an entity might contain:

```text
volume = 500ml
```

The query layer supports attribute-style queries using:

```text
key:value
```

The attribute indexes include:

- Attribute key index
- Attribute value index
- Attribute key/value pair index

An exact key/value match receives stronger evidence than a value-only match.

---

## Source Search

Entities can be associated with a registered source.

For example:

```text
Open Food Facts
```

can be registered as a source and associated with multiple entities.

Searching for a source can therefore return all entities originating from that source.

Example:

```powershell
cargo eir search data/entities.eir "Open Food Facts"
```

Possible results:

```text
Golden Grain Crunch
Aurora Tomato Soup
FrostPeak Vegetable Pizza
Crystal Spring Water
...
```

The results contain a `Source` signal explaining the match.

---

## Relationship Search

EIR can resolve entities through relationships.

For example:

```text
FizzBerry Spark
    │
    └── InstanceOf → Drink
```

Searching for the related entity can therefore produce both the directly matched entity and entities connected to it.

Relationship results include information about:

- Relationship type
- Target entity
- Source entity

The relationship index makes target-based relationship lookup efficient.

---

# Search Ranking

Search results are ranked using multiple signals.

The base confidence of the current search signals is approximately:

| Match Type   | Base Score |
| ------------ | ---------: |
| Exact Alias  |       1.00 |
| Prefix Alias |       0.80 |
| Relationship |       0.70 |
| Fuzzy Alias  |       0.60 |
| Token        |       0.50 |
| Tag          |       0.40 |
| Attribute    |       0.30 |
| Source       |       0.20 |

These values are base signal strengths rather than a guarantee that every final result will have exactly that score.

Multiple signals can contribute to the same candidate.

For example:

```text
FizzBerry Spark score=1.00

Signals:
Token
PrefixAlias
ExactAlias
FuzzyAlias
```

This means the entity was independently supported by several search strategies.

The ranker can also apply bonuses when an entity receives multiple matching signals.

---

# Insert

Insert an entity into an existing EIR database.

## Usage

```powershell
eir insert <DATABASE> <ENTITY>
```

The input is an entity JSON document.

Example:

```powershell
cargo eir insert data/entities.eir fixtures/test-entity.json
```

A test entity might contain:

```json
{
  "id": 9100,
  "aliases": ["Test Berry"],
  "tags": ["test"],
  "attributes": [],
  "relationships": [],
  "sources": [
    {
      "provider": "test source"
    }
  ]
}
```

After insertion, the entity can be inspected:

```powershell
cargo eir inspect data/entities.eir --entity 9100
```

Result:

```text
Entity: 9100

Names:
Test Berry

Tags:
test

Attributes:

Relationships:

Sources:
test source
```

The entity is also immediately searchable:

```powershell
cargo eir search data/entities.eir "Test Berry"
```

Example:

```text
Search: Test Berry

Test Berry score=1.00
Signals:
ExactAlias
PrefixAlias
FuzzyAlias
```

### Index updates

After an insertion, EIR rebuilds its search indexes from the current entity collection.

This guarantees that all indexes remain consistent with the stored documents.

The rebuilt indexes include:

- Alias index
- Prefix trie
- BK-tree
- Token inverted index
- Tag posting lists
- Source posting lists
- Attribute indexes
- Relationship indexes

---

# Remove

Remove an entity from an EIR database.

## Usage

```powershell
eir remove <DATABASE> --entity <ID>
```

Example:

```powershell
cargo eir remove data/entities.eir --entity 9100
```

After removal, inspecting the entity returns:

```text
Error: Entity not found
```

Searching for the removed entity also returns no results:

```powershell
cargo eir search data/entities.eir "Test Berry"
```

Result:

```text
Search: Test Berry
```

with no matching entities.

### Index updates

Removing an entity also rebuilds the search indexes.

This is important because an entity must disappear not only from the entity collection but from every search structure that could return it.

The operation therefore maintains the invariant:

```text
database.entities
        │
        │
        ▼
   IndexBuilder
        │
        ▼
    all indexes
```

The remaining entities are preserved and remain searchable.

---

# Persistence

Database mutations are persisted to the `.eir` database file.

For example:

```powershell
cargo eir insert data/entities.eir fixtures/test-entity.json
cargo eir remove data/entities.eir --entity 9100
```

The resulting database can be closed and reopened without losing the mutation.

This is verified by loading the database again and constructing a resolver from the persisted indexes and entity documents.

---

# Index Management

EIR maintains several specialized indexes.

Current index structures include:

| Index                     | Purpose                        |
| ------------------------- | ------------------------------ |
| Alias index               | Exact alias lookup             |
| Trie                      | Prefix matching                |
| BK-tree                   | Fuzzy alias matching           |
| Inverted index            | Token lookup                   |
| Tag posting list          | Tag → entities                 |
| Source posting list       | Source → entities              |
| Attribute key index       | Attribute key → entities       |
| Attribute value index     | Attribute value → entities     |
| Attribute pair index      | Key/value → entities           |
| Relationship posting list | Relationship target → entities |

The indexes are rebuilt from the database's entity documents when required.

This currently favors correctness and deterministic index construction over incremental mutation complexity.

---

# Generate

Generate test datasets.

Example:

```powershell
cargo eir generate
```

Generated data is useful for:

- Development
- Unit testing
- CLI testing
- Search experiments
- Benchmarking
- Testing relationships
- Testing tags and attributes

---

# Shell Completions

Generate shell completion scripts.

Supported shells include:

- Bash
- Zsh
- Fish
- PowerShell
- Elvish

Example:

```powershell
eir completions PowerShell
```

---

# Development Workflow

A typical EIR development workflow is:

## 1. Generate data

```powershell
cargo eir generate
```

## 2. Build a database

```powershell
cargo eir build `
  --input fixtures/entities.json `
  --output data/entities.eir
```

## 3. Inspect an entity

```powershell
cargo eir inspect data/entities.eir --entity 1
```

## 4. Check database statistics

```powershell
cargo eir stats data/entities.eir
```

## 5. Test search

```powershell
cargo eir search data/entities.eir "FizzBerry"
```

## 6. Insert an entity

```powershell
cargo eir insert data/entities.eir fixtures/test-entity.json
```

## 7. Verify the inserted entity

```powershell
cargo eir inspect data/entities.eir --entity 9100
```

## 8. Verify searchability

```powershell
cargo eir search data/entities.eir "Test Berry"
```

## 9. Remove the entity

```powershell
cargo eir remove data/entities.eir --entity 9100
```

## 10. Verify removal

```powershell
cargo eir inspect data/entities.eir --entity 9100
cargo eir search data/entities.eir "Test Berry"
```

The first command should report:

```text
Error: Entity not found
```

and the search should return no results.

---

# Testing

The EIR core library contains tests covering:

- Database creation
- Entity storage
- Entity lookup
- Entity insertion
- Entity removal
- Search index updates
- Persistence after removal
- Exact alias search
- Prefix search
- Fuzzy search
- Token search
- Tag search
- Attribute queries
- Relationship queries
- Search planning
- Search ranking
- Registry persistence
- Database serialization

Run the complete EIR core test suite with:

```powershell
cargo test -p eir-core
```

A successful run should report all tests passing.

---

# Architecture

The CLI sits on top of the EIR core database and search engine.

The important separation is:

```text
eir-cli
   │
   ▼
eir-core
   │
   ├── Database
   │      ├── Entities
   │      ├── Registries
   │      └── Indexes
   │
   ├── Query
   │      ├── Parser
   │      ├── Intent
   │      └── Planner
   │
   └── Search
          ├── Operators
          ├── Candidates
          ├── Ranker
          └── Results
```

The CLI is therefore an interface to the database and search infrastructure rather than containing the search implementation itself.

---

# Current Limitations

The current implementation favors correctness and simple deterministic rebuilding.

In particular:

- Database mutations rebuild indexes rather than updating every index incrementally.
- Search output is currently human-readable rather than a stable machine-readable format.
- Interactive search is not yet implemented.
- Index benchmarking is not yet exposed as a dedicated CLI workflow.
- Relationship graph visualization is not yet available from the CLI.

These can be added as the underlying EIR APIs stabilize.

---

# Future Improvements

Potential future CLI features include:

- Interactive search mode
- JSON output mode
- Export resolved entities
- Index benchmarking
- Search explanation mode
- Relationship graph inspection
- Database migration tools
- Import/export commands
- Batch insert/remove operations
- Index rebuild commands
- Database validation and integrity checks

---

# Summary

The EIR CLI provides a developer interface for working with local entity-resolution databases.

It currently supports:

- Building databases
- Inspecting entities
- Searching entities
- Inserting entities
- Removing entities
- Inspecting database statistics
- Working with aliases and tokens
- Searching tags
- Searching attributes
- Searching sources
- Resolving relationships
- Generating test datasets
- Generating shell completions
- Testing and debugging search behavior

The CLI provides a complete development loop:

```text
Build
  ↓
Inspect
  ↓
Search
  ↓
Insert / Remove
  ↓
Rebuild indexes
  ↓
Search again
  ↓
Verify persistence
```

This makes `eir` the primary development and data-management tool for building and maintaining EIR datasets and search infrastructure.
