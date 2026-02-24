#include "rublas.h"
#include <stdio.h>
#include <stdlib.h>

#define INIT_ARRAY(array, size, value)                                         \
  {                                                                            \
    for (size_t _i = 0; _i < (size); ++_i)                                     \
      (array)[_i] = (value);                                                   \
  }

#define INIT_ARRAY2(array, size, value1, value2)                               \
  {                                                                            \
    for (size_t _i = 0; _i < (size); ++_i)                                     \
      (array)[_i] = ((_i & 1u) == 0 ? (value1) : (value2));                    \
  }

#define START_TEST(name)                                                       \
  { fprintf(stdout, "test %s ... ", (name)); }

#define END_TEST()                                                             \
  { fprintf(stdout, "\033[0;32mok\033[0m\n"); }

static inline void print_usize(usize_t val) { fprintf(stderr, "%zu", val); }

static inline void print_f32(f32_t val) { fprintf(stderr, "%f", val); }

static inline void print_f64(f64_t val) { fprintf(stderr, "%f", val); }

static inline void print_cf32(cf32_t val) {
  fprintf(stderr, "%f + %fi", crealf(val), cimagf(val));
}

static inline void print_cf64(cf64_t val) {
  fprintf(stderr, "%f + %fi", crealf(val), cimagf(val));
}

static inline void print_unknown(void) { fprintf(stderr, "<unknown type>"); }

#define PRINT_VALUE(val)                                                       \
  _Generic((val),                                                              \
      usize_t: print_usize,                                                    \
      f32_t: print_f32,                                                        \
      f64_t: print_f64,                                                        \
      cf32_t: print_cf32,                                                      \
      cf64_t: print_cf64,                                                      \
      default: print_unknown)(val)

#define ASSERT_EQ(a, b)                                                        \
  {                                                                            \
    if ((a) != (b)) {                                                          \
      fprintf(stdout, "\033[0;31mFAILED\033[0m\n");                            \
      fprintf(stderr, "\n---- error ----\n");                                  \
      fprintf(stderr, "assertion `left == right` failed\n");                   \
      fprintf(stderr, "  left: ");                                             \
      PRINT_VALUE(a);                                                          \
      fprintf(stderr, "\n");                                                   \
      fprintf(stderr, " right: ");                                             \
      PRINT_VALUE(b);                                                          \
      fprintf(stderr, "\n\n");                                                 \
      exit(1);                                                                 \
    }                                                                          \
  }

#define ASSERT_EQ_ARRAY(a, b, size)                                            \
  {                                                                            \
    for (size_t i = 0; i < (size); ++i) {                                      \
      if ((a)[i] != (b)[i]) {                                                  \
        fprintf(stdout, "\033[0;31mFAILED\033[0m\n");                          \
        fprintf(stderr, "\n---- error ----\n");                                \
        fprintf(stderr, "assertion `left == right` failed ay %s:%d\n",         \
                __FILE__, __LINE__);                                           \
        fprintf(stderr, "  left: [");                                          \
        for (size_t l = 0; l < (size); ++l) {                                  \
          if (l > 0) {                                                         \
            fprintf(stderr, ", ");                                             \
          }                                                                    \
          PRINT_VALUE(a[l]);                                                   \
        }                                                                      \
        fprintf(stderr, "]\n");                                                \
        fprintf(stderr, " right: [");                                          \
        for (size_t r = 0; r < (size); ++r) {                                  \
          if (r > 0) {                                                         \
            fprintf(stderr, ", ");                                             \
          }                                                                    \
          PRINT_VALUE(b[r]);                                                   \
        }                                                                      \
        fprintf(stderr, "]\n\n");                                              \
        exit(1);                                                               \
      }                                                                        \
    }                                                                          \
  }
