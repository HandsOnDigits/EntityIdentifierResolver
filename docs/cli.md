# EIR CLI Documentation

## Overview

`eir` is the command-line interface for the Entity Identifier Resolver (EIR).

The CLI provides tools for creating, building, inspecting, searching, modifying, merging, and maintaining EIR databases.

EIR is a local, strongly typed entity search and resolution engine. Entities can contain:

* aliases
* tags
* attributes
* sources
* relationships

The CLI is built on top of `eir-core`. The CLI does not implement the search engine itself; it exposes the database and search functionality provided by the core library.

> **Warning:** EIR databases are not encrypted. Do not use EIR to store sensitive data unless the surrounding storage environment provides appropriate protection.

---

# Workspace

The current workspace is split into:

```text
EntityIdentifierResolver/
├── crates/
│   ├── eir-core/
│   ├── eir-cli/
│   └── eir-version/
├── apps/
├── docs/
└── fixtures/
```

The main responsibilities are:

| Component     | Responsibility                                                       |
| ------------- | -------------------------------------------------------------------- |
| `eir-core`    | Database, storage, indexing, query planning, search and entity types |
| `eir-cli`     | Command-line interface                                               |
| `eir-version` | Database/version/merge-related APIs                                  |
| `fixtures`    | Test and example entity datasets                                     |

The workspace is defined in the root `Cargo.toml`.

---

# Running the CLI

During development, run the CLI through Cargo:

```powershell
cargo run -p eir-cli -- <COMMAND>
```

If the repository's Cargo alias is available:

```powershell
cargo eir <COMMAND>
```

Once installed, the command is simply:

```powershell
eir <COMMAND>
```

Show general help:

```powershell
cargo eir --help
```

Show command-specific help:

```powershell
cargo eir <COMMAND> --help
```

---

# Command Overview

The current CLI provides the following operations:

```text
init
build
stats
inspect
search
insert
remove
update
compact
merge
server
completions
```

The CLI entry point dispatches these commands into the corresponding command implementations.

Some commands are intended primarily for local development and database maintenance.

---

# Database Layout

An EIR database is no longer just a standalone serialized file.

The `.eir` file is the **logical database identity**. Physical storage lives alongside it.

A typical database looks like:

```text
nutrition/
├── nutrition.eir
├── eir.toml
├── segments/
│   ├── 0000.deir
│   └── 0001.deir
└── wal/
```

The layout is managed by `DatabasePaths` and `StorageConfig`.

### Files and directories

| Path         | Purpose                          |
| ------------ | -------------------------------- |
| `<name>.eir` | Logical database identity        |
| `eir.toml`   | Database/storage configuration   |
| `segments/`  | Persistent DEIR storage segments |
| `wal/`       | Write-ahead log                  |

The database root is derived from the location of the `.eir` file when an existing database is opened.

---

# Initialize a Database

Create an empty EIR database with:

```powershell
cargo eir init data nutrition
```

This creates:

```text
data/
└── nutrition/
    ├── nutrition.eir
    ├── eir.toml
    ├── segments/
    └── wal/
```

The command accepts:

```text
eir init <PARENT> <NAME>
```

For example:

```powershell
cargo eir init data foods
```

The `init` command delegates database creation to `eir_core::engine::Engine::create`.

---

# Build

Build a database from an entity dataset:

```powershell
cargo eir build `
  --input fixtures/entities.json `
  --database data/entities
```

The command accepts:

```text
eir build --input <INPUT> --database <DATABASE>
```

The build pipeline:

1. Loads the entity input.
2. Maps input entities into EIR's internal representation.
3. Builds registries for tags, sources, attribute keys and relationship types.
4. Builds the search indexes.
5. Creates the `Database`.
6. Writes the resulting database to storage.

The resulting database contains:

```text
Database
├── Entities
├── Tag Registry
├── Source Registry
├── Attribute-Key Registry
├── Relationship-Type Registry
└── Indexes
```

---

# Entity Input

EIR works with entity input documents.

A minimal entity can look like:

```json
{
  "id": 9100,
  "aliases": ["Test Berry"],
  "tags": ["test"],
  "sources": [
    {
      "provider": "Test Source",
      "verified": false
    }
  ],
  "attributes": [],
  "relationships": []
}
```

Entities are identified by their `EntityID`.

Entity data is converted into an internal `EntityDocument` when inserted into the database.

Registries convert repeated strings such as tags, source names, attribute keys and relationship types into compact internal IDs.

---

# Stats

Show database statistics:

```powershell
cargo eir stats data/entities/entities.eir
```

The command reports:

```text
Database Statistics
===================

Entities:           46
Tags:               43
Sources:            2
Attributes:         15
Relationship Types: 5

Indexes
-------
Aliases:       89
Trie:          89
Fuzzy Aliases: 92
Tokens:        74
Tags:          43
Sources:       2
Relationships: 39
```

The exact numbers depend on the database.

Statistics are obtained from the in-memory `Database` representation and its indexes.

---

# Inspect

Inspect one or more entities:

```powershell
cargo eir inspect data/entities/entities.eir 9100
```

Multiple IDs can be supplied:

```powershell
cargo eir inspect data/entities/entities.eir 1000 1001 1002
```

The command accepts:

```text
eir inspect <DATABASE> <ENTITY>...
```

Use verbose mode to display internal registry IDs:

```powershell
cargo eir inspect data/entities/entities.eir 9100 --verbose
```

Example:

```text
Entity: 9100

Names:
  Test Berry

Tags:
  test

Attributes:

Relationships:

Sources:
  Test Source
```

Verbose output can additionally show internal IDs for tags, attributes, relationships and sources.

If an entity is missing, the command reports:

```text
Entity 9100 not found
```

Inspection reads the entity directly from the database maintained by `Engine`.

---

# Search

Search a database:

```powershell
cargo eir search data/entities/entities.eir "FizzBerry"
```

The optional result limit defaults to 10:

```powershell
cargo eir search data/entities/entities.eir "FizzBerry" --limit 20
```

The command accepts:

```text
eir search <DATABASE> <QUERY> [--limit <N>]
```

Search results include:

* entity alias
* score
* search signals
* explanations

Example:

```text
Search: FizzBerry

FizzBerry Spark score=1.00
  Signals:
    ExactAlias
    PrefixAlias
    Token
  Why:
    ExactAlias { ... }

FizzBerry Energy Blast score=0.78
  Signals:
    PrefixAlias
    Token
  Why:
    PrefixAlias { ... }
```

The CLI obtains results through `Engine::search()`, which delegates to the resolver/search implementation in `eir-core`.

---

# Search Architecture

Search is separated into several stages.

At a high level:

```text
Query
  │
  ▼
Parser
  │
  ▼
Intent / Filters
  │
  ▼
Planner
  │
  ▼
Search Executor
  │
  ├── Exact Alias
  ├── Prefix Alias
  ├── Fuzzy Alias
  ├── Token
  ├── Tag
  ├── Attribute
  └── Relationship
        │
        ▼
    Candidates
        │
        ▼
      Ranker
        │
        ▼
     Results
```

The query layer contains parsing, intent and filtering components. The search layer contains planning, execution, candidates, signals, ranking and results.

This means the CLI is intentionally thin:

```text
eir-cli
   │
   ▼
Engine
   │
   ├── Database
   │
   └── Resolver
          │
          └── Search pipeline
```

---

# Search Indexes

EIR maintains specialized indexes for different search operations.

The current database index set includes:

| Index                | Purpose                   |
| -------------------- | ------------------------- |
| Alias index          | Exact alias lookup        |
| Trie                 | Prefix alias lookup       |
| BK-tree              | Fuzzy alias lookup        |
| Inverted index       | Token lookup              |
| Tag posting lists    | Tag → entities            |
| Source posting lists | Source → entities         |
| Attribute indexes    | Attribute lookup          |
| Relationship indexes | Relationship-based lookup |

The index structures are represented by `Indexes` and built by `IndexBuilder`.

---

# Insert

Insert entities into an existing database:

```powershell
cargo eir insert data/entities/entities.eir fixtures/test-entity.json
```

The input file may contain one or more entities.

The CLI:

1. Opens the existing database.
2. Loads the entities.
3. Inserts each entity through `Engine`.
4. Flushes the database.

An entity with an existing ID is rejected.

For example:

```text
EntityAlreadyExists
```

The database itself also rebuilds its indexes after an insertion so the resolver reflects the new entity immediately.

---

# Update

Replace an existing entity:

```powershell
cargo eir update `
  data/entities/entities.eir `
  9100 `
  --input fixtures/test-entity-updated.json
```

The update input must contain exactly one entity.

The entity ID in the input must match the ID supplied to the command.

For example:

```text
eir update <DATABASE> <ENTITY> --input <JSON>
```

The command validates:

* exactly one input entity
* matching entity IDs

before applying the update. It then flushes the updated database.

---

# Remove

Remove one or more entities:

```powershell
cargo eir remove data/entities/entities.eir 9100
```

Multiple IDs can be supplied:

```powershell
cargo eir remove data/entities/entities.eir 9100 9101 9102
```

The command accepts:

```text
eir remove <DATABASE> <ENTITY>...
```

Each entity is removed through `Engine::remove()`.

After removal, the database rebuilds its search indexes so the removed entity is no longer searchable.

---

# Persistence and WAL

Database mutations are written through the storage backend.

The current engine uses a write-ahead log (WAL) for mutations.

Conceptually:

```text
Insert / Remove
      │
      ▼
     WAL
      │
      ▼
  Database
      │
      ▼
  Resolver
```

When a database is opened, EIR:

1. Resolves the physical database layout.
2. Loads the persisted database snapshot.
3. Replays pending WAL operations.
4. Reconstructs the resolver.

This allows unflushed mutations to be recovered after reopening the database.

---

# Flush

`Engine::flush()` writes the current database snapshot to persistent storage.

The CLI mutation commands use this mechanism after their operation completes.

For example:

```text
insert
  │
  ├── append WAL operation
  ├── update in-memory database
  ├── rebuild resolver
  └── flush
```

The WAL is truncated after a successful flush.

---

# Compact

Compaction rewrites persistent storage to reclaim space from obsolete data.

Run:

```powershell
cargo eir compact data/entities/entities.eir
```

Output:

```text
Database Compacted
==================

Before:    23.5 KB
After:     18.7 KB
Reclaimed: 4.8 KB
Savings:   20.4%
```

The actual values depend on the database.

Compaction operates on the physical database storage, including its segments.

It does not change the logical entity contents.

The command reports:

* storage size before compaction
* storage size after compaction
* reclaimed space
* percentage savings

The underlying `Engine::compact()` operation rewrites storage from the current database record.

---

# Merge

Merge two EIR databases into a new output database:

```powershell
cargo eir merge `
  data/left/left.eir `
  data/right/right.eir `
  data/merged/merged.eir
```

The command accepts:

```text
eir merge <LEFT> <RIGHT> <OUTPUT>
```

The merge operation:

1. Loads both databases.
2. Validates entity IDs.
3. Combines the entity collections.
4. Remaps registry IDs where necessary.
5. Rebuilds indexes.
6. Writes the merged database.

Duplicate entity IDs are rejected rather than silently overwritten.

The merge operation is designed to be atomic: validation occurs before the destination database is modified.

The CLI reports:

```text
Merge complete.
Entities added: 42
Entities skipped: 0
```

---

# Server

The CLI contains the server command structure:

```powershell
cargo eir server start data/entities/entities.eir
```

The server command accepts:

```text
eir server start <DATABASE> [--host <HOST>] [--port <PORT>]
```

Defaults:

```text
host = 127.0.0.1
port = 8765
```

The CLI also exposes:

```powershell
cargo eir server close
```

The server command is currently part of the CLI surface, but server functionality should be considered separate from the core local database/search architecture.

---

# Shell Completions

Generate shell completion scripts with:

```powershell
cargo eir completions powershell
```

Supported shells are:

```text
bash
zsh
fish
powershell
elvish
```

The CLI uses `clap_complete` to generate completions from the command definition.

---

# Typical Development Workflow

A typical workflow for creating and testing a database is:

## 1. Initialize

```powershell
cargo eir init data nutrition
```

## 2. Build

```powershell
cargo eir build `
  --input fixtures/entities.json `
  --database data/nutrition
```

## 3. Inspect

```powershell
cargo eir inspect `
  data/nutrition/nutrition.eir `
  1000
```

## 4. Check statistics

```powershell
cargo eir stats data/nutrition/nutrition.eir
```

## 5. Search

```powershell
cargo eir search `
  data/nutrition/nutrition.eir `
  "FizzBerry"
```

## 6. Insert

```powershell
cargo eir insert `
  data/nutrition/nutrition.eir `
  fixtures/test-entity.json
```

## 7. Update

```powershell
cargo eir update `
  data/nutrition/nutrition.eir `
  9100 `
  --input fixtures/test-entity-updated.json
```

## 8. Remove

```powershell
cargo eir remove `
  data/nutrition/nutrition.eir `
  9100
```

## 9. Compact

```powershell
cargo eir compact `
  data/nutrition/nutrition.eir
```

---

# Architecture

The current EIR architecture is intentionally divided into layers.

```text
                    eir-cli
                       │
                       ▼
                    Engine
                       │
             ┌─────────┴─────────┐
             ▼                   ▼
         Database             Resolver
             │                   │
       ┌─────┼─────┐       ┌─────┴─────┐
       │     │     │       │           │
   Entities Registries Indexes       Search
                                       │
                                ┌──────┼──────┐
                                ▼      ▼      ▼
                             Query  Planner  Ranker
```

## `eir-cli`

Provides the command-line interface.

## `Engine`

Provides the high-level database lifecycle API:

* create
* open
* search
* inspect
* insert
* remove
* update
* flush
* compact

The engine owns the storage backend, database and resolver.

## `Database`

Stores:

```text
Entities
Registries
Indexes
```

The registries include:

* tags
* sources
* attribute keys
* relationship types

The database can rebuild indexes from the entity collection.

## Storage

The storage subsystem contains:

```text
Backend
DEIR segments
Segment manager
Store
WAL
Registries
Posting lists
Indexes
```

These components provide the persistent storage layer beneath the logical database.

## Query

The query layer is responsible for turning user input into structured search intent and filters.

```text
Query
├── Parser
├── Intent
├── Filters
└── Types
```

## Search

The search layer executes the planned operations and ranks candidates.

```text
Search
├── Planner
├── Executor
├── Operators
├── Candidate
├── Signal
├── Ranker
├── Context
└── Result
```

---

# Database Lifecycle

The current lifecycle is:

```text
                 Entity Dataset
                       │
                       ▼
                     Build
                       │
                       ▼
                 Logical Database
                       │
              ┌────────┼────────┐
              │        │        │
           Search   Inspect   Stats
              │
        ┌─────┼─────────────┐
        │     │             │
      Insert Update       Remove
        │     │             │
        └─────┼─────────────┘
              │
              ▼
             WAL
              │
              ▼
           Snapshot
              │
              ▼
          Compaction
```

The important distinction is that **logical database state** and **physical storage** are separate concerns.

The `.eir` path identifies the database, while the storage backend manages snapshots, segments and WAL data.

---

# Index Consistency

EIR currently favors deterministic index rebuilding over maintaining complex incremental index mutations.

When an entity is inserted or removed:

```text
Database entities
       │
       ▼
 IndexBuilder
       │
       ▼
 All search indexes
```

This ensures the resolver is reconstructed from the current entity collection.

The indexes therefore remain consistent with the database documents after mutations.

---

# Testing

The core library contains tests for the database, persistence, storage and search layers.

Run the core tests:

```powershell
cargo test -p eir-core
```

Run the CLI tests:

```powershell
cargo test -p eir-cli
```

Run the complete workspace:

```powershell
cargo test --workspace
```

Important areas covered by the current test suite include:

* database creation
* database opening
* persistence
* WAL recovery
* insertion
* removal
* compaction
* merging
* duplicate entity detection
* index rebuilding
* exact alias search
* prefix search
* fuzzy search
* token search
* tags
* attributes
* relationships
* query planning
* ranking

---

# Current Limitations

EIR is still under active development.

The current repository describes the CLI, storage and search layers as functional, while the server remains incomplete.

In particular:

* The server interface exists, but the server implementation is not yet complete.
* Search output is currently intended for human-readable CLI use.
* Database mutations rebuild search indexes.
* The CLI does not yet expose every internal storage/index operation.
* The database format is still evolving.

Applications depending on EIR should therefore expect database and API formats to evolve until a stable format/versioning policy is established.

---

# Summary

The EIR CLI is a management and development interface over the EIR engine.

Its primary responsibilities are:

```text
Create
  ↓
Build
  ↓
Inspect / Stats / Search
  ↓
Insert / Update / Remove
  ↓
Flush / Recover
  ↓
Compact
  ↓
Merge
```

The CLI itself is intentionally thin. The important functionality lives in `eir-core`:

```text
eir-cli
   │
   ▼
Engine
   │
   ├── Database
   │     ├── Entities
   │     ├── Registries
   │     └── Indexes
   │
   ├── Storage
   │     ├── Segments
   │     └── WAL
   │
   └── Resolver
         │
         ├── Query
         ├── Planner
         ├── Executor
         └── Ranker
```

This architecture allows the same database and search engine to be used by the CLI, future server interfaces, and applications embedding EIR directly.
