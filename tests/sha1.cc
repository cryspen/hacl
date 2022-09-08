/*
 *    Copyright 2022 Cryspen Sarl
 *
 *    Licensed under the Apache License, Version 2.0 or MIT.
 *    - http://www.apache.org/licenses/LICENSE-2.0
 *    - http://opensource.org/licenses/MIT
 */

#include <fstream>
#include <gtest/gtest.h>
#include <nlohmann/json.hpp>

#include "EverCrypt_Hash.h"
#include "Hacl_Hash_Base.h"
#include "Hacl_Spec.h"
#include "Hacl_Streaming_SHA1.h"
#include "config.h"
#include "evercrypt.h"
#include "util.h"

using json = nlohmann::json;
using namespace std;

class TestCase
{
public:
  bytes msg;
  bytes md;
};

ostream&
operator<<(ostream& os, const TestCase& test)
{
  os << "TestCase {" << endl
     << "\t.msg = " << bytes_to_hex(test.msg) << endl
     << "\t.md = " << bytes_to_hex(test.md) << endl
     << "}" << endl;
  return os;
}

vector<TestCase>
read_json(string test_file)
{
  // Read JSON test vector
  ifstream json_test_file(test_file);
  json test_vectors;
  json_test_file >> test_vectors;

  vector<TestCase> tests_out;

  // Read tests
  for (auto& test : test_vectors.items()) {
    auto test_value = test.value();
    auto msg = from_hex(test_value["msg"]);
    auto md = from_hex(test_value["md"]);
    tests_out.push_back({ msg, md });
  }

  return tests_out;
}

class Sha1 : public ::testing::TestWithParam<tuple<TestCase, vector<size_t>>>
{};

TEST_P(Sha1, KAT)
{
  TestCase test;
  vector<size_t> lengths;
  tie(test, lengths) = GetParam();

  bytes digest(test.md.size());

  // Init
  Hacl_Streaming_SHA2_state_sha2_224* state =
    Hacl_Streaming_SHA1_legacy_create_in_sha1();
  Hacl_Streaming_SHA1_legacy_init_sha1(state);

  // Update
  for (auto chunk : split_by_index_list(test.msg, lengths)) {
    Hacl_Streaming_SHA1_legacy_update_sha1(state, chunk.data(), chunk.size());
  }

  // Finish
  Hacl_Streaming_SHA1_legacy_finish_sha1(state, digest.data());
  Hacl_Streaming_SHA1_legacy_free_sha1(state);

  EXPECT_EQ(test.md, digest) << bytes_to_hex(test.md) << endl
                             << bytes_to_hex(digest) << endl;
}

// ----- EverCrypt -------------------------------------------------------------

typedef EverCryptSuite<tuple<TestCase, vector<size_t>>> EverCryptSuiteTestCase;

TEST_P(EverCryptSuiteTestCase, HashTest)
{
  EverCryptConfig config;
  tuple<TestCase, vector<size_t>> test_tuple;
  tie(config, test_tuple) = this->GetParam();
  TestCase test;
  vector<size_t> lengths;
  tie(test, lengths) = test_tuple;

  {
    bytes got_digest(
      Hacl_Hash_Definitions_hash_len(Spec_Hash_Definitions_SHA1));

    EverCrypt_Hash_hash(Spec_Hash_Definitions_SHA1,
                        got_digest.data(),
                        test.msg.data(),
                        test.msg.size());
    ASSERT_EQ(test.md, got_digest);
  }

  {
    bytes got_digest(
      Hacl_Hash_Definitions_hash_len(Spec_Hash_Definitions_SHA1));

    // Use a typedef?
    Hacl_Streaming_Functor_state_s___EverCrypt_Hash_state_s____* state =
      EverCrypt_Hash_Incremental_create_in(Spec_Hash_Definitions_SHA1);
    EverCrypt_Hash_Incremental_init(state);

    for (auto chunk : split_by_index_list(test.msg, lengths)) {
      EverCrypt_Hash_Incremental_update(state, chunk.data(), chunk.size());
    }

    EverCrypt_Hash_Incremental_finish(state, got_digest.data());
    EverCrypt_Hash_Incremental_free(state);

    ASSERT_EQ(test.md, got_digest);
  }
}

// -----------------------------------------------------------------------------

INSTANTIATE_TEST_SUITE_P(
  Sha1Cryspen,
  Sha1,
  ::testing::Combine(::testing::ValuesIn(read_json("cryspen_sha1.json")),
                     ::testing::ValuesIn(make_lengths())));

INSTANTIATE_TEST_SUITE_P(
  Sha1CAVPShort,
  Sha1,
  ::testing::Combine(::testing::ValuesIn(read_json("sha1-short.json")),
                     ::testing::ValuesIn(make_lengths())));

INSTANTIATE_TEST_SUITE_P(
  Sha1CAVPLong,
  Sha1,
  ::testing::Combine(::testing::ValuesIn(read_json("sha1-long.json")),
                     ::testing::ValuesIn(make_lengths())));

// ----- EverCrypt -------------------------------------------------------------

// SHA1 does not use additional hardware features.
// Nothing to disable here.
vector<EverCryptConfig>
generate_sha1_configs()
{
  vector<EverCryptConfig> configs;

  configs.push_back(EverCryptConfig{
    .disable_adx = false,
    .disable_aesni = false,
    .disable_avx = false,
    .disable_avx2 = false,
    .disable_avx512 = false,
    .disable_bmi2 = false,
    .disable_movbe = false,
    .disable_pclmulqdq = false,
    .disable_rdrand = false,
    .disable_shaext = false,
    .disable_sse = false,
  });

  return configs;
}

INSTANTIATE_TEST_SUITE_P(
  Sha1Cryspen,
  EverCryptSuiteTestCase,
  ::testing::Combine(
    ::testing::ValuesIn(generate_sha1_configs()),
    ::testing::Combine(::testing::ValuesIn(read_json("cryspen_sha1.json")),
                       ::testing::ValuesIn(make_lengths()))));

INSTANTIATE_TEST_SUITE_P(
  Sha1CAVPShort,
  EverCryptSuiteTestCase,
  ::testing::Combine(
    ::testing::ValuesIn(generate_sha1_configs()),
    ::testing::Combine(::testing::ValuesIn(read_json("sha1-short.json")),
                       ::testing::ValuesIn(make_lengths()))));

INSTANTIATE_TEST_SUITE_P(
  Sha1CAVPLong,
  EverCryptSuiteTestCase,
  ::testing::Combine(
    ::testing::ValuesIn(generate_sha1_configs()),
    ::testing::Combine(::testing::ValuesIn(read_json("sha1-long.json")),
                       ::testing::ValuesIn(make_lengths()))));