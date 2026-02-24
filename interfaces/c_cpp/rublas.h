#ifndef RUBLAS_H
#define RUBLAS_H
#include <complex.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * types
 */

typedef size_t usize_t;

typedef float f32_t;

typedef double f64_t;

typedef float complex cf32_t;

typedef double complex cf64_t;

/*
 * Level 1 BLAS
 */

// axpy
void axpy_f32(const usize_t n, const f32_t alpha, const f32_t *x,
              const usize_t inc_x, f32_t *y, const usize_t inc_y);
void axpy_f64(const usize_t n, const f64_t alpha, const f64_t *x,
              const usize_t inc_x, f64_t *y, const usize_t inc_y);
void axpy_cf32(const usize_t n, const cf32_t *alpha, const cf32_t *x,
               const usize_t inc_x, cf32_t *y, const usize_t inc_y);
void axpy_cf64(const usize_t n, const cf64_t *alpha, const cf64_t *x,
               const usize_t inc_x, cf64_t *y, const usize_t inc_y);

// scal
void scal_f32(const usize_t n, const f32_t alpha, f32_t *x,
              const usize_t inc_x);
void scal_f64(const usize_t n, const f64_t alpha, f64_t *x,
              const usize_t inc_x);
void scal_cf32(const usize_t n, const cf32_t *alpha, cf32_t *x,
               const usize_t inc_x);
void scal_cf64(const usize_t n, const cf64_t *alpha, cf64_t *x,
               const usize_t inc_x);

#ifdef __cplusplus
}
#endif

#endif /* RUBLAS_H */
