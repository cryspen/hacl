/* 
  This file was generated by KaRaMeL <https://github.com/FStarLang/karamel>
  KaRaMeL invocation: /Users/franziskus/repos/eurydice//eurydice ../libcrux_kyber.llbc
  F* version: d0aa54cf
  KaRaMeL version: 11522064
 */

#ifndef __internal_core_H
#define __internal_core_H

#include "eurydice_glue.h"

static inline int64_t core_convert_num__i64_59__from(int32_t x0);

#define core_option_None 0
#define core_option_Some 1

typedef uint8_t core_option_Option__size_t_tags;

typedef struct core_option_Option__size_t_s
{
  core_option_Option__size_t_tags tag;
  size_t f0;
}
core_option_Option__size_t;

static inline uint16_t core_num__u16_7__wrapping_add(uint16_t x0, uint16_t x1);

static inline uint8_t core_num__u8_6__wrapping_sub(uint8_t x0, uint8_t x1);

#define core_num__u32_8__BITS 32

typedef struct core_option_Option__uint32_t_s
{
  core_option_Option__size_t_tags tag;
  uint32_t f0;
}
core_option_Option__uint32_t;

typedef struct core_option_Option__int32_t_s
{
  core_option_Option__size_t_tags tag;
  int32_t f0;
}
core_option_Option__int32_t;


#define __internal_core_H_DEFINED
#endif
