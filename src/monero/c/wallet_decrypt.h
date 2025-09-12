#ifndef MONERO_WALLET_DECRYPT_H
#define MONERO_WALLET_DECRYPT_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Decrypts a Monero wallet file (.keys) using the provided password.
 * 
 * @param wallet_data Pointer to the full .keys file content
 * @param wallet_len Length of wallet_data in bytes
 * @param password Null-terminated UTF-8 string
 * @param out_len Pointer to size_t to receive length of decrypted buffer
 * 
 * @return Pointer to a heap-allocated buffer containing decrypted bytes.
 *         Must be freed using free_decrypted_wallet().
 */
uint8_t* decrypt_wallet_bytes(const uint8_t* wallet_data, size_t wallet_len, const char* password, size_t* out_len);

/**
 * Frees memory returned by decrypt_wallet_bytes().
 */
void free_decrypted_wallet(uint8_t* ptr);

#ifdef __cplusplus
}
#endif

#endif // MONERO_WALLET_DECRYPT_H