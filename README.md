# hashcore

A high-performance, C-compatible shared library for **xxHash64** file hashing, optimized for macOS.

## Features
- **Streaming I/O**: Low memory footprint regardless of file size.
- **Progress Callbacks**: Real-time feedback for UI integration.
- **Thread-Safe Cancellation**: Stop hashing instantly from any thread.
- **FFI Ready**: Designed for seamless Swift and C integration.

## Installation
Build the dynamic library using Cargo:
```bash
cargo build --release
````

The output will be in `target/release/libhashcore.dylib`.

## Quick Start (C)

```c
#include "hashcore.h"

void on_progress(uint64_t bytes, uint64_t total, void *data) {
    printf("Progress: %llu/%llu\n", bytes, total);
}

int main() {
    HashContext *ctx = hash_ffi_init("large_file.bin", 65536);
    if (ctx) {
        if (hash_ffi_process(ctx, on_progress, NULL) == 0) {
            uint64_t result = hash_ffi_finalize(ctx);
            printf("Hash: %llu\n", result);
        } else {
            hash_ffi_cleanup(ctx);
        }
    }
    return 0;
}
````

## License

#### Copyright (c) 2026 Mullet. All rights reserved.

This software is provided for personal or internal use.
- **Prohibited**: Commercial sale, redistribution, or integration into other commercial projects without express written permission from **Mullet**.
- See the `LICENSE` file for full details.