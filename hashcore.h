#ifndef HASHCORE_H
#define HASHCORE_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// --- Types & Error Codes ---

typedef struct HashContext HashContext;
typedef void (*CProgressCallback)(uint64_t processed, uint64_t total, void *user_data);

#define HASHCORE_SUCCESS           0
#define HASHCORE_ERR_INVALID_CTX  -1
#define HASHCORE_ERR_NOT_FOUND    -2
#define HASHCORE_ERR_CANCELLED    -4
#define HASHCORE_ERR_IO           -5

// --- Streaming API ---

/**
 * Initializes a new hash context. 
 * Returns NULL if file cannot be opened.
 */
HashContext *hash_ffi_init(const char *path, uint32_t buf_size);

/**
 * Processes the file and updates the hash.
 * Returns 0 on success, negative error code on failure/cancellation.
 */
int hash_ffi_process(HashContext *ctx, CProgressCallback cb, void *user_data);

/**
 * Finalizes computation and returns the result. 
 * Frees the context memory.
 */
uint64_t hash_ffi_finalize(HashContext *ctx);

/**
 * Signals an ongoing process to stop. 
 * Safe to call from any thread.
 */
void hash_ffi_cancel(HashContext *ctx);

/**
 * Frees the context without returning a hash. 
 * Use for error cleanup.
 */
void hash_ffi_cleanup(HashContext *ctx);

#ifdef __cplusplus
}
#endif

#endif