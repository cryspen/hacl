#include "libcrux_hacl_glue.h"
#include "Hacl_Hash_SHA3.h"
#include "libcrux_kyber.h"

#ifdef HACL_CAN_COMPILE_VEC256
#include "Hacl_Hash_SHA3_Simd256.h"
#else
#include "Hacl_Hash_SHA3_Scalar.h"
#endif

bool
libcrux_platform_simd256_support(void)
{
  // TODO: Replace this with HACL platform support.
  return false;
}

inline void
libcrux_digest_shake256_(size_t len, Eurydice_slice input, uint8_t* out)
{
  Hacl_Hash_SHA3_shake256_hacl(input.len, input.ptr, (uint32_t)len, out);
}

inline void
libcrux_digest_shake128_(size_t len, Eurydice_slice input, uint8_t* out)
{
  Hacl_Hash_SHA3_shake128_hacl(input.len, input.ptr, (uint32_t)len, out);
}

inline void
libcrux_digest_sha3_512(Eurydice_slice x0, uint8_t x1[64U])
{
  Hacl_Hash_SHA3_sha3_512(x1, x0.ptr, (uint32_t)x0.len);
}

inline void
libcrux_digest_sha3_256(Eurydice_slice x0, uint8_t x1[32U])
{
  Hacl_Hash_SHA3_sha3_256(x1, x0.ptr, (uint32_t)x0.len);
}

inline libcrux_digest_incremental_x4_Shake128StateX4
libcrux_digest_incremental_x4__libcrux__digest__incremental_x4__Shake128StateX4__new(
  void)
{
#ifdef HACL_CAN_COMPILE_VEC256
  return (libcrux_digest_incremental_x4_Shake128StateX4){
    .x4 =
      (Lib_IntVector_Intrinsics_vec256*)Hacl_Hash_SHA3_Simd256_state_malloc()
  };
#else
  uint64_t* st0 = Hacl_Hash_SHA3_Scalar_state_malloc();
  uint64_t* st1 = Hacl_Hash_SHA3_Scalar_state_malloc();
  uint64_t* st2 = Hacl_Hash_SHA3_Scalar_state_malloc();
  uint64_t* st3 = Hacl_Hash_SHA3_Scalar_state_malloc();
  return (libcrux_digest_incremental_x4_Shake128StateX4){
    .st0 = st0, .st1 = st1, .st2 = st2, .st3 = st3
  };
#endif
}

void
libcrux_digest_incremental_x4__libcrux__digest__incremental_x4__Shake128StateX4__absorb_final(
  libcrux_digest_incremental_x4_Shake128StateX4* x0,
  Eurydice_slice x1[4U])
{
#ifdef HACL_CAN_COMPILE_VEC256
  Hacl_Hash_SHA3_Simd256_shake128_absorb_final(
    x0->x4, x1[0].ptr, x1[1].ptr, x1[2].ptr, x1[3].ptr, x1[0].len);
#else
  Hacl_Hash_SHA3_Scalar_shake128_absorb_final(x0->st0, x1[0].ptr, x1[0].len);
  Hacl_Hash_SHA3_Scalar_shake128_absorb_final(x0->st1, x1[1].ptr, x1[1].len);
  Hacl_Hash_SHA3_Scalar_shake128_absorb_final(x0->st2, x1[2].ptr, x1[2].len);
  Hacl_Hash_SHA3_Scalar_shake128_absorb_final(x0->st3, x1[3].ptr, x1[3].len);
#endif
}

inline void
libcrux_digest_incremental_x4__libcrux__digest__incremental_x4__Shake128StateX4__squeeze_blocks_f(
  libcrux_digest_incremental_x4_Shake128StateX4* x1,
  size_t block_len,
  uint8_t* output)
{
#ifdef HACL_CAN_COMPILE_VEC256
  uint8_t* tmp = aligned_alloc(32, block_len);
  Hacl_Hash_SHA3_Simd256_shake128_squeeze_nblocks(
    x1->x4, output, output + block_len, output + 2 * block_len, tmp, block_len);
  free(tmp);
#else
  Hacl_Hash_SHA3_Scalar_shake128_squeeze_nblocks(x1->st0, output, block_len);
  Hacl_Hash_SHA3_Scalar_shake128_squeeze_nblocks(
    x1->st1, output + block_len, block_len);
  Hacl_Hash_SHA3_Scalar_shake128_squeeze_nblocks(
    x1->st2, output + 2 * block_len, block_len);
#endif
}

inline void
libcrux_digest_incremental_x4__libcrux__digest__incremental_x4__Shake128StateX4__free(
  libcrux_digest_incremental_x4_Shake128StateX4 x0)
{
#ifdef HACL_CAN_COMPILE_VEC256
  Hacl_Hash_SHA3_Simd256_state_free(x0.x4);
#else
  Hacl_Hash_SHA3_Scalar_state_free(x0.st0);
  Hacl_Hash_SHA3_Scalar_state_free(x0.st1);
  Hacl_Hash_SHA3_Scalar_state_free(x0.st2);
  Hacl_Hash_SHA3_Scalar_state_free(x0.st3);
#endif
}