# EIR Data Model

This document describes the data model used by Entity Identifier Resolver (EIR).

The model is centered around an **Entity Document**. An entity has one stable identifier and can be described using aliases, tags, sources, attributes, and relationships.

The database then builds registries and search indexes from these entities.

---

## Model Overview

```mermaid
flowchart TB
    ENTITY["EntityDocument"]

    ID["EntityID"]
    ALIASES["Aliases"]
    TAGS["Tags"]
    SOURCES["Sources"]
    ATTRIBUTES["Attributes"]
    RELATIONSHIPS["Relationships"]

    ENTITY --> ID
    ENTITY --> ALIASES
    ENTITY --> TAGS
    ENTITY --> SOURCES
    ENTITY --> ATTRIBUTES
    ENTITY --> RELATIONSHIPS
```

At its simplest:

```text
EntityDocument
│
├── EntityID
├── aliases
├── tags
├── sources
├── attributes
└── relationships
```

---

# Entity

An entity represents the thing EIR is trying to identify.

Example:

```json
{
  "id": 1001,
  "aliases": [
    "FizzBerry Spark",
    "FizzBerry",
    "Berry Spark"
  ],
  "tags": [
    "drink",
    "berry"
  ],
  "sources": [
    {
      "provider": "Open Food Facts",
      "verified": true
    }
  ],
  "attributes": [],
  "relationships": []
}
```

The important property is that **the entity ID is stable while the descriptive data can evolve**.

---

# Entity ID

Every entity has an `EntityID`.

```text
EntityID
   │
   └── uniquely identifies an entity
```

For example:

```text
1001 → FizzBerry Spark
1002 → FizzBerry Energy
1003 → Aurora Foods
```

The ID is used throughout the database, indexes, relationships, and search results.

---

# Aliases

Aliases are names by which an entity may be known.

```mermaid
flowchart LR
    ENTITY["Entity 1001"]

    A1["FizzBerry Spark"]
    A2["FizzBerry"]
    A3["Berry Spark"]

    ENTITY --> A1
    ENTITY --> A2
    ENTITY --> A3
```

An entity can therefore have multiple searchable identifiers:

```text
1001
├── FizzBerry Spark
├── FizzBerry
└── Berry Spark
```

Aliases are used by several search mechanisms:

```text
Aliases
   │
   ├── Exact matching
   ├── Prefix matching
   ├── Fuzzy matching
   └── Token matching
```

---

# Tags

Tags describe categories or classifications associated with an entity.

```text
Entity 1001

Tags
├── drink
└── berry
```

Tags are indexed independently from aliases.

This allows queries such as:

```text
berry
tag:drink
```

to participate in search without requiring the tag to be an alias.

---

# Sources

Sources describe where information about an entity came from.

Example:

```json
{
  "provider": "Open Food Facts",
  "verified": true
}
```

An entity may have multiple sources:

```mermaid
flowchart LR
    ENTITY["Entity 1001"]

    OFF["Open Food Facts"]
    REG["Manufacturer Registry"]
    OTHER["Other Source"]

    ENTITY --> OFF
    ENTITY --> REG
    ENTITY --> OTHER
```

Sources are registered and indexed so that entities can be filtered or searched by provenance.

---

# Attributes

Attributes provide structured properties of an entity.

Conceptually:

```text
Entity
│
└── Attributes
    ├── key
    └── value
```

For example:

```text
Entity 1001

Attributes
├── brand = "FizzBerry"
├── category = "Soft Drink"
└── country = "Denmark"
```

Attributes are different from aliases.

An alias is intended to identify an entity.

An attribute describes something about the entity.

---

# Relationships

Relationships connect one entity to another.

```mermaid
flowchart LR
    A["Entity A"]

    R["Relationship"]

    B["Entity B"]

    A --> R
    R --> B
```

For example:

```text
FizzBerry Spark
      │
      │ manufactured_by
      ▼
FizzBerry Foods
```

Relationships consist conceptually of:

```text
source entity
relationship type
target entity
```

This allows EIR to represent connections between entities rather than treating every entity as an isolated document.

---

# Complete Entity Model

Putting the pieces together:

```mermaid
flowchart TB
    E["EntityDocument"]

    ID["EntityID"]

    ALIAS["Aliases"]
    TAG["Tags"]
    SOURCE["Sources"]
    ATTRIBUTE["Attributes"]
    REL["Relationships"]

    E --> ID
    E --> ALIAS
    E --> TAG
    E --> SOURCE
    E --> ATTRIBUTE
    E --> REL

    REL --> TARGET["Other Entity"]
```

A more concrete example:

```text
┌─────────────────────────────────────────┐
│ Entity 1001                             │
│                                         │
│ FizzBerry Spark                         │
│                                         │
│ Aliases                                 │
│   • FizzBerry Spark                     │
│   • FizzBerry                           │
│   • Berry Spark                         │
│                                         │
│ Tags                                    │
│   • drink                               │
│   • berry                               │
│                                         │
│ Sources                                 │
│   • Open Food Facts                     │
│                                         │
│ Attributes                              │
│   • brand = FizzBerry                   │
│                                         │
│ Relationships                           │
│   • manufactured_by → Entity 2001       │
└─────────────────────────────────────────┘
```

---

# Registries

EIR uses registries for values that occur repeatedly across entities.

The main registries include:

```mermaid
flowchart TB
    DB["Database"]

    TAGS["Tag Registry"]
    SOURCES["Source Registry"]
    ATTR_KEYS["Attribute-Key Registry"]
    REL_TYPES["Relationship-Type Registry"]

    DB --> TAGS
    DB --> SOURCES
    DB --> ATTR_KEYS
    DB --> REL_TYPES
```

Instead of storing repeated strings everywhere, the database can assign internal identifiers.

For example:

```text
Tag Registry

0 → drink
1 → berry
2 → food
```

An entity can then refer to:

```text
Entity 1001
tags = [0, 1]
```

This reduces duplication and allows the indexes to operate on compact IDs.

---

# Database Model

Entities and registries are combined into the logical database.

```mermaid
flowchart TB
    DB["Database"]

    ENTITIES["Entity Documents"]

    TAGS["Tag Registry"]
    SOURCES["Source Registry"]
    ATTR["Attribute-Key Registry"]
    REL["Relationship-Type Registry"]

    INDEXES["Indexes"]

    DB --> ENTITIES
    DB --> TAGS
    DB --> SOURCES
    DB --> ATTR
    DB --> REL
    DB --> INDEXES
```

Conceptually:

```text
Database
│
├── Entities
│
├── Registries
│   ├── Tags
│   ├── Sources
│   ├── Attribute Keys
│   └── Relationship Types
│
└── Indexes
```

---

# From Model to Indexes

The database model is the source of truth.

Indexes are derived structures.

```mermaid
flowchart LR
    ENTITIES["Entity Documents"]

    BUILDER["IndexBuilder"]

    ALIAS["Alias Index"]
    TRIE["Prefix Trie"]
    FUZZY["Fuzzy Index"]
    TOKEN["Token Index"]
    TAG["Tag Index"]
    SOURCE["Source Index"]
    ATTRIBUTE["Attribute Index"]
    RELATIONSHIP["Relationship Index"]

    ENTITIES --> BUILDER

    BUILDER --> ALIAS
    BUILDER --> TRIE
    BUILDER --> FUZZY
    BUILDER --> TOKEN
    BUILDER --> TAG
    BUILDER --> SOURCE
    BUILDER --> ATTRIBUTE
    BUILDER --> RELATIONSHIP
```

This distinction is important:

> **Entities are the data. Indexes are derived search structures.**

If an entity changes, the indexes must represent the new database state.

---

# Search Model

Search operates against the derived indexes.

```mermaid
flowchart TB
    QUERY["User Query"]

    PARSER["Query Parser"]
    PLAN["Query Planner"]

    INDEXES["Search Indexes"]

    CANDIDATES["Candidates"]

    SIGNALS["Signals"]

    RANKER["Ranker"]

    RESULTS["Search Results"]

    QUERY --> PARSER
    PARSER --> PLAN
    PLAN --> INDEXES
    INDEXES --> CANDIDATES
    CANDIDATES --> SIGNALS
    SIGNALS --> RANKER
    RANKER --> RESULTS
```

A result can therefore be understood as:

```text
Query
  │
  ▼
Candidate Entity
  │
  ├── matched alias
  ├── matched token
  ├── matched tag
  ├── matched property
  └── matched relationship
       │
       ▼
     Score
       │
       ▼
    Result
```

---

# Full Data Flow

The complete relationship between the model, database, indexes, and search engine is:

```mermaid
flowchart LR
    INPUT["Entity Documents"]

    DB["Database"]

    REG["Registries"]

    INDEX["Indexes"]

    QUERY["Query"]

    PLAN["Planner"]

    EXEC["Executor"]

    CAND["Candidates"]

    RANK["Ranker"]

    RESULT["Results"]

    INPUT --> DB

    DB --> REG
    DB --> INDEX

    QUERY --> PLAN
    PLAN --> INDEX
    INDEX --> EXEC

    EXEC --> CAND
    CAND --> RANK
    RANK --> RESULT
```

---

# Storage Model

The logical model is persisted separately from the search process.

```mermaid
flowchart TB
    ENGINE["Engine"]

    DB["Database"]
    RESOLVER["Resolver"]
    BACKEND["Backend"]

    WAL["Write-Ahead Log"]
    SEGMENTS["DEIR Segments"]

    ENGINE --> DB
    ENGINE --> RESOLVER
    ENGINE --> BACKEND

    BACKEND --> WAL
    BACKEND --> SEGMENTS

    DB --> RESOLVER
```

The engine therefore provides the bridge between:

```text
Logical Model
     │
     ▼
Database
     │
     ▼
Persistent Storage
```

while the resolver provides:

```text
Database
   │
   ▼
Indexes
   │
   ▼
Search
```

---

# Mutation Model

An entity mutation follows the database lifecycle.

```mermaid
sequenceDiagram
    participant Client
    participant Engine
    participant WAL
    participant Database
    participant Resolver

    Client->>Engine: Insert / Update / Remove
    Engine->>WAL: Record mutation
    Engine->>Database: Apply mutation
    Engine->>Resolver: Rebuild search state
    Engine-->>Client: Success

    Client->>Engine: Flush
    Engine->>Database: Persist state
    Engine->>WAL: Finalize / truncate
```

The important invariant is:

```text
Database state
      │
      ▼
Search indexes
      │
      ▼
Resolver state
```

The resolver should represent the current database.

---

# Model Invariants

The model is built around several important invariants.

### Entity IDs are unique

```text
EntityID → one entity
```

An insert using an existing ID is rejected.

### Entity IDs are stable

Changing an entity's aliases, tags, attributes, or relationships does not change its identity.

### Indexes are derived

Indexes should represent the current entity collection rather than becoming an independent source of truth.

### Registries are shared

Repeated tag, source, attribute-key, and relationship-type values are represented through registries.

### Relationships reference entities

A relationship connects entities rather than embedding a second copy of the target entity.

---

# Mental Model

The simplest way to think about EIR is:

```text
                    ENTITY
                      │
        ┌─────────────┼─────────────┐
        │             │             │
      Names        Metadata      Relations
        │             │             │
     Aliases    Tags / Sources   Entity → Entity
        │        Attributes         │
        │             │             │
        └─────────────┼─────────────┘
                      │
                      ▼
                    INDEX
                      │
                      ▼
                    SEARCH
                      │
                      ▼
                   RESULTS
```

Or, even more simply:

> **Entities are the source of truth. Registries make repeated values compact. Indexes make the entities searchable. The resolver turns queries into ranked entity matches.**
