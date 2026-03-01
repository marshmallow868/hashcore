#ifndef HASHCORE_H
#define HASHCORE_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct HashContext HashContext;
typedef void (*CProgressCallback)(uint64_t processed, uint64_t total, void *user_data);

#define HASHCORE_SUCCESS           0
#define HASHCORE_ERR_INVALID_CTX  -1
#define HASHCORE_ERR_NOT_FOUND    -2
#define HASHCORE_ERR_CANCELLED    -4
#define HASHCORE_ERR_IO           -5

HashContext *hash_ffi_init(const char *path, uint32_t buf_size);
int hash_ffi_process(HashContext *ctx, CProgressCallback cb, void *user_data);
uint64_t hash_ffi_finalize(HashContext *ctx);
void hash_ffi_cancel(HashContext *ctx);
void hash_ffi_cleanup(HashContext *ctx);

#ifdef __cplusplus
}
#endif

#endif