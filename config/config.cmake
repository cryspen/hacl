set(SOURCES_std ${PROJECT_SOURCE_DIR}/src/Hacl_Ed25519.c ${PROJECT_SOURCE_DIR}/src/Hacl_Kremlib.c ${PROJECT_SOURCE_DIR}/src/Hacl_Streaming_SHA2.c ${PROJECT_SOURCE_DIR}/src/Hacl_Hash_SHA2.c ${PROJECT_SOURCE_DIR}/src/Hacl_Curve25519_51.c ${PROJECT_SOURCE_DIR}/src/Hacl_Hash_Blake2.c ${PROJECT_SOURCE_DIR}/src/Lib_Memzero0.c ${PROJECT_SOURCE_DIR}/src/Hacl_Chacha20Poly1305_32.c ${PROJECT_SOURCE_DIR}/src/Hacl_Chacha20.c ${PROJECT_SOURCE_DIR}/src/Hacl_Poly1305_32.c ${PROJECT_SOURCE_DIR}/src/Hacl_P256.c ${PROJECT_SOURCE_DIR}/src/Hacl_Spec.c ${PROJECT_SOURCE_DIR}/src/Hacl_SHA3.c ${PROJECT_SOURCE_DIR}/src/Hacl_HMAC.c ${PROJECT_SOURCE_DIR}/src/Hacl_Hash_SHA1.c ${PROJECT_SOURCE_DIR}/src/Hacl_HKDF.c ${PROJECT_SOURCE_DIR}/src/Hacl_RSAPSS.c ${PROJECT_SOURCE_DIR}/src/Hacl_Bignum.c)
set(SOURCES_vec256 ${PROJECT_SOURCE_DIR}/src/Hacl_Hash_Blake2b_256.c ${PROJECT_SOURCE_DIR}/src/Hacl_Streaming_Blake2b_256.c ${PROJECT_SOURCE_DIR}/src/Hacl_Chacha20Poly1305_256.c)
set(SOURCES_vec128 ${PROJECT_SOURCE_DIR}/src/Hacl_Hash_Blake2s_128.c ${PROJECT_SOURCE_DIR}/src/Hacl_Streaming_Blake2s_128.c ${PROJECT_SOURCE_DIR}/src/Hacl_Chacha20Poly1305_128.c)
set(SOURCES_vale ${PROJECT_SOURCE_DIR}/src/Hacl_Curve25519_64.c)
set(ALGORITHMS ed25519 blake2 chacha20poly1305 curve25519 p256 sha3 hmac hkdf rsapss)
set(INCLUDE_PATHS ${PROJECT_SOURCE_DIR}/include ${PROJECT_SOURCE_DIR}/kremlin/include/ ${PROJECT_SOURCE_DIR}/kremlin/kremlib/dist/minimal)
set(TEST_SOURCES ${PROJECT_SOURCE_DIR}/tests/blake2b.cc ${PROJECT_SOURCE_DIR}/tests/blake2s.cc ${PROJECT_SOURCE_DIR}/tests/p256.cc ${PROJECT_SOURCE_DIR}/tests/chacha20poly1305.cc ${PROJECT_SOURCE_DIR}/tests/ed25519.cc)
set(ALGORITHM_TEST_FILES TEST_FILES_blake2 TEST_FILES_p256 TEST_FILES_chacha20poly1305 TEST_FILES_ed25519)
set(TEST_FILES_blake2 blake2b.cc blake2s.cc)
set(TEST_FILES_p256 p256.cc)
set(TEST_FILES_chacha20poly1305 chacha20poly1305.cc)
set(TEST_FILES_ed25519 ed25519.cc)
