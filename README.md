# Hash Table Implementation

A hash table that serves requests through a **Posix shm**.

## Quick start

```bash
cargo run -p server -- -n 10
cargo run -p client

# using make
make server n=20
make client
```

## Overview

- **IPC Mechanism**: The POSIX shared memory API allows processes to communicate information by sharing a region of memory.
- **Collision Resolution**: Separate chaining
- The Hash table supports the following operations:
  - get
  - upsert
  - remove

### Assumptions

- Buffer has a fixed size, no reallocations
- All messages are the same size
