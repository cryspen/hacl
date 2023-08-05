/* MIT License
 *
 * Copyright (c) 2016-2022 INRIA, CMU and Microsoft Corporation
 * Copyright (c) 2022-2023 HACL* Contributors
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */


#include "Hacl_AES_256_GCM_NI.h"

void Hacl_AES_256_GCM_NI_aes256_gcm_init(Lib_IntVector_Intrinsics_vec128 *ctx, uint8_t *key)
{
  uint8_t gcm_key[16U] = { 0U };
  uint8_t nonce0[12U] = { 0U };
  Lib_IntVector_Intrinsics_vec128 *aes_ctx = ctx;
  Lib_IntVector_Intrinsics_vec128 *gcm_ctx = ctx + (uint32_t)16U;
  Hacl_AES_256_CTR32_NI_aes256_init(aes_ctx, key, nonce0);
  Hacl_AES_256_CTR32_NI_aes256_key_block(gcm_key, aes_ctx, (uint32_t)0U);
  Hacl_Gf128_NI_gcm_init(gcm_ctx, gcm_key);
}

void
Hacl_AES_256_GCM_NI_aes256_gcm_encrypt(
  Lib_IntVector_Intrinsics_vec128 *ctx,
  uint32_t len,
  uint8_t *out,
  uint8_t *text,
  uint32_t aad_len,
  uint8_t *aad,
  uint32_t iv_len,
  uint8_t *iv
)
{
  uint32_t ctr;
  uint8_t tag_mix0[16U] = { 0U };
  uint8_t gcm_key[16U] = { 0U };
  uint8_t tag_iv[16U] = { 0U };
  uint8_t size_iv[16U] = { 0U };
  uint8_t tag_mix1[16U] = { 0U };
  if (iv_len == (uint32_t)12U)
  {
    Lib_IntVector_Intrinsics_vec128 *aes_ctx = ctx;
    Hacl_AES_256_CTR32_NI_aes256_set_nonce(aes_ctx, iv);
    Hacl_AES_256_CTR32_NI_aes256_key_block(tag_mix0, aes_ctx, (uint32_t)1U);
    ctx[21U] = Lib_IntVector_Intrinsics_vec128_load128_le(tag_mix0);
    ctr = (uint32_t)2U;
  }
  else
  {
    Lib_IntVector_Intrinsics_vec128 *aes_ctx = ctx;
    Lib_IntVector_Intrinsics_vec128 *gcm_ctx = ctx + (uint32_t)16U;
    Lib_IntVector_Intrinsics_vec128_store_be(gcm_key, gcm_ctx[4U]);
    Hacl_Gf128_NI_ghash(tag_iv, iv_len, iv, gcm_key);
    store64_be(size_iv + (uint32_t)8U, (uint64_t)(iv_len * (uint32_t)8U));
    KRML_MAYBE_FOR16(i,
      (uint32_t)0U,
      (uint32_t)16U,
      (uint32_t)1U,
      size_iv[i] = tag_iv[i] ^ size_iv[i];);
    Hacl_Gf128_NI_ghash(tag_iv, (uint32_t)16U, size_iv, gcm_key);
    Hacl_AES_256_CTR32_NI_aes256_set_nonce(aes_ctx, tag_iv);
    uint32_t u = load32_be(tag_iv + (uint32_t)12U);
    uint32_t ctr0 = u;
    Hacl_AES_256_CTR32_NI_aes256_key_block(tag_mix1, aes_ctx, ctr0);
    ctx[21U] = Lib_IntVector_Intrinsics_vec128_load128_le(tag_mix1);
    ctr = ctr0 + (uint32_t)1U;
  }
  uint8_t *cip = out;
  Lib_IntVector_Intrinsics_vec128 *aes_ctx = ctx;
  Hacl_AES_256_CTR32_NI_aes256_ctr(len, cip, text, aes_ctx, ctr);
  Lib_IntVector_Intrinsics_vec128 *gcm_ctx = ctx + (uint32_t)16U;
  Lib_IntVector_Intrinsics_vec128 tag_mix = ctx[21U];
  gcm_ctx[0U] = Lib_IntVector_Intrinsics_vec128_zero;
  Hacl_Gf128_NI_gcm_update_padded(gcm_ctx, aad_len, aad);
  Hacl_Gf128_NI_gcm_update_padded(gcm_ctx, len, cip);
  uint8_t tmp[16U] = { 0U };
  store64_be(tmp, (uint64_t)(aad_len * (uint32_t)8U));
  store64_be(tmp + (uint32_t)8U, (uint64_t)(len * (uint32_t)8U));
  Hacl_Gf128_NI_gcm_update_blocks(gcm_ctx, (uint32_t)16U, tmp);
  Hacl_Gf128_NI_gcm_emit(tmp, gcm_ctx);
  Lib_IntVector_Intrinsics_vec128 tmp_vec = Lib_IntVector_Intrinsics_vec128_load128_le(tmp);
  Lib_IntVector_Intrinsics_vec128
  tmp_vec1 = Lib_IntVector_Intrinsics_vec128_xor(tmp_vec, tag_mix);
  Lib_IntVector_Intrinsics_vec128_store128_le(out + len, tmp_vec1);
}

bool
Hacl_AES_256_GCM_NI_aes256_gcm_decrypt(
  Lib_IntVector_Intrinsics_vec128 *ctx,
  uint32_t len,
  uint8_t *out,
  uint8_t *cipher,
  uint32_t aad_len,
  uint8_t *aad,
  uint32_t iv_len,
  uint8_t *iv
)
{
  uint8_t scratch[18U] = { 0U };
  uint8_t *text = scratch;
  uint8_t *result = scratch + (uint32_t)17U;
  uint8_t *ciphertext = cipher;
  uint8_t *tag = cipher + len;
  uint32_t ctr;
  uint8_t tag_mix0[16U] = { 0U };
  uint8_t gcm_key[16U] = { 0U };
  uint8_t tag_iv[16U] = { 0U };
  uint8_t size_iv[16U] = { 0U };
  uint8_t tag_mix1[16U] = { 0U };
  if (iv_len == (uint32_t)12U)
  {
    Lib_IntVector_Intrinsics_vec128 *aes_ctx = ctx;
    Hacl_AES_256_CTR32_NI_aes256_set_nonce(aes_ctx, iv);
    Hacl_AES_256_CTR32_NI_aes256_key_block(tag_mix0, aes_ctx, (uint32_t)1U);
    ctx[21U] = Lib_IntVector_Intrinsics_vec128_load128_le(tag_mix0);
    ctr = (uint32_t)2U;
  }
  else
  {
    Lib_IntVector_Intrinsics_vec128 *aes_ctx = ctx;
    Lib_IntVector_Intrinsics_vec128 *gcm_ctx = ctx + (uint32_t)16U;
    Lib_IntVector_Intrinsics_vec128_store_be(gcm_key, gcm_ctx[4U]);
    Hacl_Gf128_NI_ghash(tag_iv, iv_len, iv, gcm_key);
    store64_be(size_iv + (uint32_t)8U, (uint64_t)(iv_len * (uint32_t)8U));
    KRML_MAYBE_FOR16(i,
      (uint32_t)0U,
      (uint32_t)16U,
      (uint32_t)1U,
      size_iv[i] = tag_iv[i] ^ size_iv[i];);
    Hacl_Gf128_NI_ghash(tag_iv, (uint32_t)16U, size_iv, gcm_key);
    Hacl_AES_256_CTR32_NI_aes256_set_nonce(aes_ctx, tag_iv);
    uint32_t u = load32_be(tag_iv + (uint32_t)12U);
    uint32_t ctr0 = u;
    Hacl_AES_256_CTR32_NI_aes256_key_block(tag_mix1, aes_ctx, ctr0);
    ctx[21U] = Lib_IntVector_Intrinsics_vec128_load128_le(tag_mix1);
    ctr = ctr0 + (uint32_t)1U;
  }
  Lib_IntVector_Intrinsics_vec128 *aes_ctx = ctx;
  Lib_IntVector_Intrinsics_vec128 *gcm_ctx = ctx + (uint32_t)16U;
  Lib_IntVector_Intrinsics_vec128 tag_mix = ctx[21U];
  gcm_ctx[0U] = Lib_IntVector_Intrinsics_vec128_zero;
  Hacl_Gf128_NI_gcm_update_padded(gcm_ctx, aad_len, aad);
  Hacl_Gf128_NI_gcm_update_padded(gcm_ctx, len, ciphertext);
  store64_be(text, (uint64_t)(aad_len * (uint32_t)8U));
  store64_be(text + (uint32_t)8U, (uint64_t)(len * (uint32_t)8U));
  Hacl_Gf128_NI_gcm_update_blocks(gcm_ctx, (uint32_t)16U, text);
  Hacl_Gf128_NI_gcm_emit(text, gcm_ctx);
  Lib_IntVector_Intrinsics_vec128 text_vec = Lib_IntVector_Intrinsics_vec128_load128_le(text);
  Lib_IntVector_Intrinsics_vec128
  text_vec1 = Lib_IntVector_Intrinsics_vec128_xor(text_vec, tag_mix);
  Lib_IntVector_Intrinsics_vec128_store128_le(text, text_vec1);
  KRML_MAYBE_FOR16(i,
    (uint32_t)0U,
    (uint32_t)16U,
    (uint32_t)1U,
    result[0U] = result[0U] | (text[i] ^ tag[i]););
  uint8_t res8 = result[0U];
  if (res8 == (uint8_t)0U)
  {
    Hacl_AES_256_CTR32_NI_aes256_ctr(len, out, ciphertext, aes_ctx, ctr);
    return true;
  }
  return false;
}

