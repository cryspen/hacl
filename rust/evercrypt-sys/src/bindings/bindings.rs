/* automatically generated by rust-bindgen 0.58.1 */

pub const EverCrypt_Error_Success: u32 = 0;
pub const EverCrypt_Error_UnsupportedAlgorithm: u32 = 1;
pub const EverCrypt_Error_InvalidKey: u32 = 2;
pub const EverCrypt_Error_AuthenticationFailure: u32 = 3;
pub const EverCrypt_Error_InvalidIVLength: u32 = 4;
pub const EverCrypt_Error_DecodeError: u32 = 5;
pub const Spec_Blake2_Blake2S: u32 = 0;
pub const Spec_Blake2_Blake2B: u32 = 1;
pub const Spec_Hash_Definitions_SHA2_224: u32 = 0;
pub const Spec_Hash_Definitions_SHA2_256: u32 = 1;
pub const Spec_Hash_Definitions_SHA2_384: u32 = 2;
pub const Spec_Hash_Definitions_SHA2_512: u32 = 3;
pub const Spec_Hash_Definitions_SHA1: u32 = 4;
pub const Spec_Hash_Definitions_MD5: u32 = 5;
pub const Spec_Hash_Definitions_Blake2S: u32 = 6;
pub const Spec_Hash_Definitions_Blake2B: u32 = 7;
pub const Spec_ECDSA_NoHash: u32 = 0;
pub const Spec_ECDSA_Hash: u32 = 1;
pub const Spec_FFDHE_FFDHE2048: u32 = 0;
pub const Spec_FFDHE_FFDHE3072: u32 = 1;
pub const Spec_FFDHE_FFDHE4096: u32 = 2;
pub const Spec_FFDHE_FFDHE6144: u32 = 3;
pub const Spec_FFDHE_FFDHE8192: u32 = 4;
pub const Spec_Agile_Cipher_AES128: u32 = 0;
pub const Spec_Agile_Cipher_AES256: u32 = 1;
pub const Spec_Agile_Cipher_CHACHA20: u32 = 2;
pub const Spec_Cipher_Expansion_Hacl_CHACHA20: u32 = 0;
pub const Spec_Cipher_Expansion_Vale_AES128: u32 = 1;
pub const Spec_Cipher_Expansion_Vale_AES256: u32 = 2;
pub const Spec_Agile_AEAD_AES128_GCM: u32 = 0;
pub const Spec_Agile_AEAD_AES256_GCM: u32 = 1;
pub const Spec_Agile_AEAD_CHACHA20_POLY1305: u32 = 2;
pub const Spec_Agile_AEAD_AES128_CCM: u32 = 3;
pub const Spec_Agile_AEAD_AES256_CCM: u32 = 4;
pub const Spec_Agile_AEAD_AES128_CCM8: u32 = 5;
pub const Spec_Agile_AEAD_AES256_CCM8: u32 = 6;
pub const Spec_Frodo_Params_SHAKE128: u32 = 0;
pub const Spec_Frodo_Params_AES128: u32 = 1;
pub type C_String_t = *const ::std::os::raw::c_char;
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_shaext() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_aesni() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_pclmulqdq() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_avx2() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_avx() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_bmi2() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_adx() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_sse() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_movbe() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_rdrand() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_avx512() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_wants_vale() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_wants_hacl() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_wants_openssl() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_wants_bcrypt() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_recall();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_init();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_avx2();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_avx();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_bmi2();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_adx();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_shaext();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_aesni();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_pclmulqdq();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_sse();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_movbe();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_rdrand();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_avx512();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_vale();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_hacl();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_openssl();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_disable_bcrypt();
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_vec128() -> bool;
}
extern "C" {
    pub fn EverCrypt_AutoConfig2_has_vec256() -> bool;
}
pub type EverCrypt_Error_error_code = u8;
pub type Spec_Blake2_alg = u8;
pub type Spec_Hash_Definitions_hash_alg = u8;
pub type Spec_ECDSA_hash_alg_ecdsa_tags = u8;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Spec_ECDSA_hash_alg_ecdsa_s {
    pub tag: Spec_ECDSA_hash_alg_ecdsa_tags,
    pub _0: Spec_Hash_Definitions_hash_alg,
}
pub type Spec_ECDSA_hash_alg_ecdsa = Spec_ECDSA_hash_alg_ecdsa_s;
pub type Spec_FFDHE_ffdhe_alg = u8;
pub type Spec_Agile_Cipher_cipher_alg = u8;
pub type Spec_Cipher_Expansion_impl = u8;
pub type Spec_Agile_AEAD_alg = u8;
pub type Spec_Frodo_Params_frodo_gen_a = u8;
extern "C" {
    pub fn EverCrypt_AEAD_uu___is_Ek(
        a: Spec_Agile_AEAD_alg,
        projectee: EverCrypt_AEAD_state_s,
    ) -> bool;
}
extern "C" {
    pub fn EverCrypt_AEAD___proj__Ek__item__impl(
        a: Spec_Agile_AEAD_alg,
        projectee: EverCrypt_AEAD_state_s,
    ) -> Spec_Cipher_Expansion_impl;
}
extern "C" {
    pub fn EverCrypt_AEAD___proj__Ek__item__ek(
        a: Spec_Agile_AEAD_alg,
        projectee: EverCrypt_AEAD_state_s,
    ) -> *mut u8;
}
extern "C" {
    pub fn EverCrypt_AEAD_alg_of_state(s: *mut EverCrypt_AEAD_state_s) -> Spec_Agile_AEAD_alg;
}
extern "C" {
    pub fn EverCrypt_AEAD_create_in(
        a: Spec_Agile_AEAD_alg,
        dst: *mut *mut EverCrypt_AEAD_state_s,
        k: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_encrypt(
        s: *mut EverCrypt_AEAD_state_s,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        plain: *mut u8,
        plain_len: u32,
        cipher: *mut u8,
        tag: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_encrypt_expand_aes128_gcm_no_check(
        k: *mut u8,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        plain: *mut u8,
        plain_len: u32,
        cipher: *mut u8,
        tag: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_encrypt_expand_aes256_gcm_no_check(
        k: *mut u8,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        plain: *mut u8,
        plain_len: u32,
        cipher: *mut u8,
        tag: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_encrypt_expand_aes128_gcm(
        k: *mut u8,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        plain: *mut u8,
        plain_len: u32,
        cipher: *mut u8,
        tag: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_encrypt_expand_aes256_gcm(
        k: *mut u8,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        plain: *mut u8,
        plain_len: u32,
        cipher: *mut u8,
        tag: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_encrypt_expand_chacha20_poly1305(
        k: *mut u8,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        plain: *mut u8,
        plain_len: u32,
        cipher: *mut u8,
        tag: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_encrypt_expand(
        a: Spec_Agile_AEAD_alg,
        k: *mut u8,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        plain: *mut u8,
        plain_len: u32,
        cipher: *mut u8,
        tag: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_decrypt(
        s: *mut EverCrypt_AEAD_state_s,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        cipher: *mut u8,
        cipher_len: u32,
        tag: *mut u8,
        dst: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_decrypt_expand_aes128_gcm_no_check(
        k: *mut u8,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        cipher: *mut u8,
        cipher_len: u32,
        tag: *mut u8,
        dst: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_decrypt_expand_aes256_gcm_no_check(
        k: *mut u8,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        cipher: *mut u8,
        cipher_len: u32,
        tag: *mut u8,
        dst: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_decrypt_expand_aes128_gcm(
        k: *mut u8,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        cipher: *mut u8,
        cipher_len: u32,
        tag: *mut u8,
        dst: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_decrypt_expand_aes256_gcm(
        k: *mut u8,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        cipher: *mut u8,
        cipher_len: u32,
        tag: *mut u8,
        dst: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_decrypt_expand_chacha20_poly1305(
        k: *mut u8,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        cipher: *mut u8,
        cipher_len: u32,
        tag: *mut u8,
        dst: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_decrypt_expand(
        a: Spec_Agile_AEAD_alg,
        k: *mut u8,
        iv: *mut u8,
        iv_len: u32,
        ad: *mut u8,
        ad_len: u32,
        cipher: *mut u8,
        cipher_len: u32,
        tag: *mut u8,
        dst: *mut u8,
    ) -> EverCrypt_Error_error_code;
}
extern "C" {
    pub fn EverCrypt_AEAD_free(s: *mut EverCrypt_AEAD_state_s);
}
extern "C" {
    pub fn EverCrypt_Curve25519_secret_to_public(pub_: *mut u8, priv_: *mut u8);
}
extern "C" {
    pub fn EverCrypt_Curve25519_scalarmult(shared: *mut u8, my_priv: *mut u8, their_pub: *mut u8);
}
extern "C" {
    pub fn EverCrypt_Curve25519_ecdh(shared: *mut u8, my_priv: *mut u8, their_pub: *mut u8)
        -> bool;
}
extern "C" {
    pub fn EverCrypt_Ed25519_sign(signature: *mut u8, secret: *mut u8, len: u32, msg: *mut u8);
}
extern "C" {
    pub fn EverCrypt_Ed25519_verify(
        pubkey: *mut u8,
        len: u32,
        msg: *mut u8,
        signature: *mut u8,
    ) -> bool;
}
extern "C" {
    pub fn EverCrypt_Ed25519_secret_to_public(output: *mut u8, secret: *mut u8);
}
extern "C" {
    pub fn EverCrypt_Ed25519_expand_keys(ks: *mut u8, secret: *mut u8);
}
extern "C" {
    pub fn EverCrypt_Ed25519_sign_expanded(signature: *mut u8, ks: *mut u8, len: u32, msg: *mut u8);
}
extern "C" {
    pub fn EverCrypt_Hash_string_of_alg(uu___: Spec_Hash_Definitions_hash_alg) -> C_String_t;
}
pub type EverCrypt_Hash_state_s_tags = u8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EverCrypt_Hash_state_s_s {
    pub tag: EverCrypt_Hash_state_s_tags,
    pub __bindgen_anon_1: EverCrypt_Hash_state_s_s__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union EverCrypt_Hash_state_s_s__bindgen_ty_1 {
    pub case_MD5_s: *mut u32,
    pub case_SHA1_s: *mut u32,
    pub case_SHA2_224_s: *mut u32,
    pub case_SHA2_256_s: *mut u32,
    pub case_SHA2_384_s: *mut u64,
    pub case_SHA2_512_s: *mut u64,
    pub case_Blake2S_s: *mut u32,
    pub case_Blake2B_s: *mut u64,
}
pub type EverCrypt_Hash_state_s = EverCrypt_Hash_state_s_s;
extern "C" {
    pub fn EverCrypt_Hash_uu___is_MD5_s(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> bool;
}
extern "C" {
    pub fn EverCrypt_Hash___proj__MD5_s__item__p(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> *mut u32;
}
extern "C" {
    pub fn EverCrypt_Hash_uu___is_SHA1_s(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> bool;
}
extern "C" {
    pub fn EverCrypt_Hash___proj__SHA1_s__item__p(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> *mut u32;
}
extern "C" {
    pub fn EverCrypt_Hash_uu___is_SHA2_224_s(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> bool;
}
extern "C" {
    pub fn EverCrypt_Hash___proj__SHA2_224_s__item__p(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> *mut u32;
}
extern "C" {
    pub fn EverCrypt_Hash_uu___is_SHA2_256_s(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> bool;
}
extern "C" {
    pub fn EverCrypt_Hash___proj__SHA2_256_s__item__p(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> *mut u32;
}
extern "C" {
    pub fn EverCrypt_Hash_uu___is_SHA2_384_s(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> bool;
}
extern "C" {
    pub fn EverCrypt_Hash___proj__SHA2_384_s__item__p(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> *mut u64;
}
extern "C" {
    pub fn EverCrypt_Hash_uu___is_SHA2_512_s(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> bool;
}
extern "C" {
    pub fn EverCrypt_Hash___proj__SHA2_512_s__item__p(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> *mut u64;
}
extern "C" {
    pub fn EverCrypt_Hash_uu___is_Blake2S_s(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> bool;
}
extern "C" {
    pub fn EverCrypt_Hash___proj__Blake2S_s__item__p(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> *mut u32;
}
extern "C" {
    pub fn EverCrypt_Hash_uu___is_Blake2B_s(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> bool;
}
extern "C" {
    pub fn EverCrypt_Hash___proj__Blake2B_s__item__p(
        uu___: Spec_Hash_Definitions_hash_alg,
        projectee: EverCrypt_Hash_state_s,
    ) -> *mut u64;
}
extern "C" {
    pub fn EverCrypt_Hash_alg_of_state(
        s: *mut EverCrypt_Hash_state_s,
    ) -> Spec_Hash_Definitions_hash_alg;
}
extern "C" {
    pub fn EverCrypt_Hash_create_in(
        a: Spec_Hash_Definitions_hash_alg,
    ) -> *mut EverCrypt_Hash_state_s;
}
extern "C" {
    pub fn EverCrypt_Hash_create(a: Spec_Hash_Definitions_hash_alg) -> *mut EverCrypt_Hash_state_s;
}
extern "C" {
    pub fn EverCrypt_Hash_init(s: *mut EverCrypt_Hash_state_s);
}
extern "C" {
    pub fn EverCrypt_Hash_update_multi_256(s: *mut u32, blocks: *mut u8, n: u32);
}
extern "C" {
    pub fn EverCrypt_Hash_update2(s: *mut EverCrypt_Hash_state_s, prevlen: u64, block: *mut u8);
}
extern "C" {
    pub fn EverCrypt_Hash_update(s: *mut EverCrypt_Hash_state_s, block: *mut u8);
}
extern "C" {
    pub fn EverCrypt_Hash_update_multi2(
        s: *mut EverCrypt_Hash_state_s,
        prevlen: u64,
        blocks: *mut u8,
        len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_update_multi(s: *mut EverCrypt_Hash_state_s, blocks: *mut u8, len: u32);
}
extern "C" {
    pub fn EverCrypt_Hash_update_last_256(
        s: *mut u32,
        input: u64,
        input_len: *mut u8,
        input_len1: u32,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_update_last2(
        s: *mut EverCrypt_Hash_state_s,
        prev_len: u64,
        last: *mut u8,
        last_len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_update_last(
        s: *mut EverCrypt_Hash_state_s,
        last: *mut u8,
        total_len: u64,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_finish(s: *mut EverCrypt_Hash_state_s, dst: *mut u8);
}
extern "C" {
    pub fn EverCrypt_Hash_free(s: *mut EverCrypt_Hash_state_s);
}
extern "C" {
    pub fn EverCrypt_Hash_copy(
        s_src: *mut EverCrypt_Hash_state_s,
        s_dst: *mut EverCrypt_Hash_state_s,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_hash_256(input: *mut u8, input_len: u32, dst: *mut u8);
}
extern "C" {
    pub fn EverCrypt_Hash_hash_224(input: *mut u8, input_len: u32, dst: *mut u8);
}
extern "C" {
    pub fn EverCrypt_Hash_hash(
        a: Spec_Hash_Definitions_hash_alg,
        dst: *mut u8,
        input: *mut u8,
        len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_hash_len(a: Spec_Hash_Definitions_hash_alg) -> u32;
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_block_len(a: Spec_Hash_Definitions_hash_alg) -> u32;
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_create_in(
        a: Spec_Hash_Definitions_hash_alg,
    ) -> *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____;
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_init(
        s: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_update(
        p: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
        data: *mut u8,
        len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_finish_md5(
        p: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
        dst: *mut u8,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_finish_sha1(
        p: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
        dst: *mut u8,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_finish_sha224(
        p: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
        dst: *mut u8,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_finish_sha256(
        p: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
        dst: *mut u8,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_finish_sha384(
        p: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
        dst: *mut u8,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_finish_sha512(
        p: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
        dst: *mut u8,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_finish_blake2s(
        p: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
        dst: *mut u8,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_finish_blake2b(
        p: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
        dst: *mut u8,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_alg_of_state(
        s: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
    ) -> Spec_Hash_Definitions_hash_alg;
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_finish(
        s: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
        dst: *mut u8,
    );
}
extern "C" {
    pub fn EverCrypt_Hash_Incremental_free(
        s: *mut Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____,
    );
}
extern "C" {
    pub fn EverCrypt_HMAC_compute_sha1(
        dst: *mut u8,
        key: *mut u8,
        key_len: u32,
        data: *mut u8,
        data_len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HMAC_compute_sha2_256(
        dst: *mut u8,
        key: *mut u8,
        key_len: u32,
        data: *mut u8,
        data_len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HMAC_compute_sha2_384(
        dst: *mut u8,
        key: *mut u8,
        key_len: u32,
        data: *mut u8,
        data_len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HMAC_compute_sha2_512(
        dst: *mut u8,
        key: *mut u8,
        key_len: u32,
        data: *mut u8,
        data_len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HMAC_compute_blake2s(
        dst: *mut u8,
        key: *mut u8,
        key_len: u32,
        data: *mut u8,
        data_len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HMAC_compute_blake2b(
        dst: *mut u8,
        key: *mut u8,
        key_len: u32,
        data: *mut u8,
        data_len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HMAC_is_supported_alg(uu___: Spec_Hash_Definitions_hash_alg) -> bool;
}
extern "C" {
    pub fn EverCrypt_HMAC_compute(
        a: Spec_Hash_Definitions_hash_alg,
        mac: *mut u8,
        key: *mut u8,
        keylen: u32,
        data: *mut u8,
        datalen: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_expand_sha1(
        okm: *mut u8,
        prk: *mut u8,
        prklen: u32,
        info: *mut u8,
        infolen: u32,
        len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_extract_sha1(
        prk: *mut u8,
        salt: *mut u8,
        saltlen: u32,
        ikm: *mut u8,
        ikmlen: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_expand_sha2_256(
        okm: *mut u8,
        prk: *mut u8,
        prklen: u32,
        info: *mut u8,
        infolen: u32,
        len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_extract_sha2_256(
        prk: *mut u8,
        salt: *mut u8,
        saltlen: u32,
        ikm: *mut u8,
        ikmlen: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_expand_sha2_384(
        okm: *mut u8,
        prk: *mut u8,
        prklen: u32,
        info: *mut u8,
        infolen: u32,
        len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_extract_sha2_384(
        prk: *mut u8,
        salt: *mut u8,
        saltlen: u32,
        ikm: *mut u8,
        ikmlen: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_expand_sha2_512(
        okm: *mut u8,
        prk: *mut u8,
        prklen: u32,
        info: *mut u8,
        infolen: u32,
        len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_extract_sha2_512(
        prk: *mut u8,
        salt: *mut u8,
        saltlen: u32,
        ikm: *mut u8,
        ikmlen: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_expand_blake2s(
        okm: *mut u8,
        prk: *mut u8,
        prklen: u32,
        info: *mut u8,
        infolen: u32,
        len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_extract_blake2s(
        prk: *mut u8,
        salt: *mut u8,
        saltlen: u32,
        ikm: *mut u8,
        ikmlen: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_expand_blake2b(
        okm: *mut u8,
        prk: *mut u8,
        prklen: u32,
        info: *mut u8,
        infolen: u32,
        len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_extract_blake2b(
        prk: *mut u8,
        salt: *mut u8,
        saltlen: u32,
        ikm: *mut u8,
        ikmlen: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_expand(
        a: Spec_Hash_Definitions_hash_alg,
        okm: *mut u8,
        prk: *mut u8,
        prklen: u32,
        info: *mut u8,
        infolen: u32,
        len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_extract(
        a: Spec_Hash_Definitions_hash_alg,
        prk: *mut u8,
        salt: *mut u8,
        saltlen: u32,
        ikm: *mut u8,
        ikmlen: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_hkdf_expand(
        a: Spec_Hash_Definitions_hash_alg,
        okm: *mut u8,
        prk: *mut u8,
        prklen: u32,
        info: *mut u8,
        infolen: u32,
        len: u32,
    );
}
extern "C" {
    pub fn EverCrypt_HKDF_hkdf_extract(
        a: Spec_Hash_Definitions_hash_alg,
        prk: *mut u8,
        salt: *mut u8,
        saltlen: u32,
        ikm: *mut u8,
        ikmlen: u32,
    );
}
extern "C" {
    pub fn Hacl_P256_ecdsa_sign_p256_sha2(
        result: *mut u8,
        mLen: u32,
        m: *mut u8,
        privKey: *mut u8,
        k: *mut u8,
    ) -> bool;
}
extern "C" {
    pub fn Hacl_P256_ecdsa_sign_p256_sha384(
        result: *mut u8,
        mLen: u32,
        m: *mut u8,
        privKey: *mut u8,
        k: *mut u8,
    ) -> bool;
}
extern "C" {
    pub fn Hacl_P256_ecdsa_sign_p256_sha512(
        result: *mut u8,
        mLen: u32,
        m: *mut u8,
        privKey: *mut u8,
        k: *mut u8,
    ) -> bool;
}
extern "C" {
    pub fn Hacl_P256_ecdsa_sign_p256_without_hash(
        result: *mut u8,
        mLen: u32,
        m: *mut u8,
        privKey: *mut u8,
        k: *mut u8,
    ) -> bool;
}
extern "C" {
    pub fn Hacl_P256_ecdsa_verif_p256_sha2(
        mLen: u32,
        m: *mut u8,
        pubKey: *mut u8,
        r: *mut u8,
        s: *mut u8,
    ) -> bool;
}
extern "C" {
    pub fn Hacl_P256_ecdsa_verif_p256_sha384(
        mLen: u32,
        m: *mut u8,
        pubKey: *mut u8,
        r: *mut u8,
        s: *mut u8,
    ) -> bool;
}
extern "C" {
    pub fn Hacl_P256_ecdsa_verif_p256_sha512(
        mLen: u32,
        m: *mut u8,
        pubKey: *mut u8,
        r: *mut u8,
        s: *mut u8,
    ) -> bool;
}
extern "C" {
    pub fn Hacl_P256_ecdsa_verif_without_hash(
        mLen: u32,
        m: *mut u8,
        pubKey: *mut u8,
        r: *mut u8,
        s: *mut u8,
    ) -> bool;
}
extern "C" {
    pub fn Hacl_P256_verify_q(pubKey: *mut u8) -> bool;
}
extern "C" {
    pub fn Hacl_P256_decompression_not_compressed_form(b: *mut u8, result: *mut u8) -> bool;
}
extern "C" {
    pub fn Hacl_P256_decompression_compressed_form(b: *mut u8, result: *mut u8) -> bool;
}
extern "C" {
    pub fn Hacl_P256_compression_not_compressed_form(b: *mut u8, result: *mut u8);
}
extern "C" {
    pub fn Hacl_P256_compression_compressed_form(b: *mut u8, result: *mut u8);
}
extern "C" {
    pub fn Hacl_P256_ecp256dh_i(result: *mut u8, scalar: *mut u8) -> bool;
}
extern "C" {
    pub fn Hacl_P256_ecp256dh_r(result: *mut u8, pubKey: *mut u8, scalar: *mut u8) -> bool;
}
extern "C" {
    pub fn Hacl_P256_is_more_than_zero_less_than_order(x: *mut u8) -> bool;
}
extern "C" {
    pub fn Hacl_SHA3_shake128_hacl(
        inputByteLen: u32,
        input: *mut u8,
        outputByteLen: u32,
        output: *mut u8,
    );
}
extern "C" {
    pub fn Hacl_SHA3_shake256_hacl(
        inputByteLen: u32,
        input: *mut u8,
        outputByteLen: u32,
        output: *mut u8,
    );
}
extern "C" {
    pub fn Hacl_SHA3_sha3_224(inputByteLen: u32, input: *mut u8, output: *mut u8);
}
extern "C" {
    pub fn Hacl_SHA3_sha3_256(inputByteLen: u32, input: *mut u8, output: *mut u8);
}
extern "C" {
    pub fn Hacl_SHA3_sha3_384(inputByteLen: u32, input: *mut u8, output: *mut u8);
}
extern "C" {
    pub fn Hacl_SHA3_sha3_512(inputByteLen: u32, input: *mut u8, output: *mut u8);
}
