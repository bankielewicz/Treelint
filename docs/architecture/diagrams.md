# Treelint Architecture Diagrams

Mermaid diagrams for Treelint architecture visualization.

---

## Component Diagram
```mermaid
graph TB
    subgraph CLI["CLI Layer"]
        Main[main.rs]
        Args[args.rs]
        SearchCmd[search.rs]
    end

    subgraph Domain["Domain Layer"]
        subgraph Parser["Parser Module"]
            Lang[Language]
            SymExt[SymbolExtractor]
            Queries[Queries]
        end

        subgraph Index["Index Module"]
            Storage[IndexStorage]
            Search[QueryFilters]
            Schema[Schema]
        end

        subgraph Output["Output Module"]
            JSON[JsonFormatter]
            Text[TextFormatter]
        end
    end

    subgraph Infra["Infrastructure Layer"]
        subgraph Daemon["Daemon Module"]
            Server[DaemonServer]
            Watcher[FileWatcher]
            Indexer[IncrementalIndexer]
            Protocol[Protocol]
        end

        SQLite[(SQLite)]
        TS[tree-sitter]
        Notify[notify]
    end

    Main --> Args
    Main --> SearchCmd
    SearchCmd --> SymExt
    SearchCmd --> Storage
    SearchCmd --> JSON
    SearchCmd --> Text

    SymExt --> Lang
    SymExt --> Queries
    SymExt --> TS

    Storage --> SQLite
    Storage --> Schema

    Server --> Protocol
    Server --> Storage
    Watcher --> Notify
    Watcher --> Indexer
    Indexer --> SymExt
    Indexer --> Storage
```

---

## Data Flow: Search Command

```mermaid
sequenceDiagram
    participant User
    participant CLI as CLI (main.rs)
    participant Cmd as SearchCommand
    participant Idx as IndexStorage
    participant Par as SymbolExtractor
    participant DB as SQLite

    User->>CLI: treelint search "validateUser"
    CLI->>Cmd: execute(SearchArgs)

    Cmd->>Idx: has_index()?
    Idx->>DB: SELECT count(*) FROM symbols

    alt Index exists
        DB-->>Idx: count > 0
        Idx-->>Cmd: true
        Cmd->>Idx: query(filters)
        Idx->>DB: SELECT * FROM symbols WHERE name LIKE ?
        DB-->>Idx: rows
        Idx-->>Cmd: Vec<Symbol>
    else Index missing
        DB-->>Idx: count = 0
        Idx-->>Cmd: false
        Cmd->>Par: extract_from_directory(.)
        Par-->>Cmd: Vec<Symbol>
        Cmd->>Idx: bulk_insert(symbols)
        Idx->>DB: INSERT INTO symbols
    end

    Cmd-->>CLI: SearchResult
    CLI-->>User: JSON/Text output
```

---

## Data Flow: Daemon File Watcher

```mermaid
sequenceDiagram
    participant FS as FileSystem
    participant W as FileWatcher
    participant H as HashCache
    participant I as IncrementalIndexer
    participant S as IndexStorage
    participant DB as SQLite

    FS->>W: file.py modified event
    W->>W: debounce (100ms window)

    W->>H: compute_hash(file.py)
    H-->>W: new_hash

    W->>H: get_cached(file.py)
    H-->>W: old_hash

    alt Hashes differ
        W->>I: reindex_file(file.py)
        I->>S: delete_by_file(file.py)
        S->>DB: DELETE WHERE file_path = ?
        I->>I: parse file (tree-sitter)
        I->>S: bulk_insert(symbols)
        S->>DB: INSERT INTO symbols
        W->>H: set_cached(file.py, new_hash)
    else Hashes match
        Note over W: Skip re-indexing (no content change)
    end
```

---

## Data Flow: Daemon IPC Request

```mermaid
sequenceDiagram
    participant C as Client
    participant S as DaemonServer
    participant P as Protocol
    participant I as IndexStorage
    participant DB as SQLite

    C->>S: Connect to socket
    C->>S: {"id":"1","method":"search","params":{"symbol":"main"}}

    S->>P: parse_request(json)
    P-->>S: Request{method: search, params}

    alt Method: search
        S->>I: query(filters)
        I->>DB: SELECT * FROM symbols WHERE ...
        DB-->>I: rows
        I-->>S: Vec<Symbol>
        S->>P: format_response(symbols)
        P-->>S: {"id":"1","result":[...],"error":null}
    else Method: status
        S->>S: get_status()
        S->>P: format_response(status)
        P-->>S: {"id":"1","result":{status},"error":null}
    else Method: unknown
        S->>P: format_error(E002)
        P-->>S: {"id":"1","result":null,"error":{"code":"E002",...}}
    end

    S-->>C: NDJSON response
```

---

## Layer Dependency Diagram

```mermaid
graph TD
    subgraph CLI["CLI Layer"]
        A[main.rs]
        B[args.rs]
        C[commands/]
    end

    subgraph App["Application Layer"]
        D[search.rs]
    end

    subgraph Domain["Domain Layer"]
        E[parser/]
        F[index/]
        G[output/]
    end

    subgraph Infra["Infrastructure Layer"]
        H[daemon/]
        I[storage.rs]
        J[tree-sitter]
        K[SQLite]
    end

    A --> B
    A --> C
    C --> D
    D --> E
    D --> F
    D --> G
    E --> J
    F --> I
    I --> K
    H --> I
    H --> E

    style CLI fill:#e1f5fe
    style App fill:#fff3e0
    style Domain fill:#e8f5e9
    style Infra fill:#fce4ec
```

---

## Module Size Distribution

```mermaid
pie title Lines of Code by Module
    "parser/symbols.rs" : 1933
    "daemon/server.rs" : 1224
    "index/storage.rs" : 1203
    "daemon/watcher.rs" : 1097
    "parser/context.rs" : 322
    "index/search.rs" : 256
    "cli/args.rs" : 156
    "Other" : 1109
```

---

## Deployment Diagram

```mermaid
graph TB
    subgraph Machine["User's Machine"]
        subgraph Tools["Development Tools"]
            AI[AI Assistant<br/>Claude Code]
            Editor[Code Editor<br/>VS Code]
        end

        subgraph Treelint["Treelint"]
            CLI[treelint CLI]
            Daemon[treelint daemon]
            Watcher[File Watcher]
        end

        subgraph Data["Data Layer"]
            Index[.treelint/index.db<br/>SQLite]
            Socket[.treelint/daemon.sock<br/>Unix Socket]
        end

        subgraph Project["Project"]
            Code[Source Files<br/>*.py, *.ts, *.rs, *.md]
        end
    end

    AI <-->|IPC| Socket
    Editor -->|File Save| Code
    CLI --> Index
    Daemon --> Index
    Daemon --> Socket
    Watcher -->|notify events| Code
    Watcher --> Daemon

    style AI fill:#bbdefb
    style Daemon fill:#c8e6c9
    style Index fill:#fff9c4
```

---

## State Machine: Daemon States

```mermaid
stateDiagram-v2
    [*] --> Starting: daemon start
    Starting --> Ready: initialization complete
    Ready --> Indexing: file change detected
    Ready --> Indexing: index command received
    Indexing --> Ready: indexing complete
    Ready --> Stopping: shutdown signal
    Stopping --> [*]: cleanup complete

    note right of Starting: Creating socket<br/>Loading index
    note right of Ready: Accepting requests<br/>Watching files
    note right of Indexing: Re-parsing files<br/>Updating DB
    note right of Stopping: Draining connections<br/>Removing socket
```

---

**Version:** 0.8.0
**Generated:** 2026-01-30
