#ifndef RUBLAS_H
#define RUBLAS_H
#include <complex.h>
#include <stddef.h>

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

// asum
f32_t asum_f32(const usize_t n, const f32_t *x, const usize_t inc_x);
f64_t asum_f64(const usize_t n, const f64_t *x, const usize_t inc_x);
f32_t asum_cf32(const usize_t n, const cf32_t *x, const usize_t inc_x);
f64_t asum_cf64(const usize_t n, const cf64_t *x, const usize_t inc_x);

// axpy
void axpy_f32(const usize_t n, const f32_t alpha, const f32_t *x,
              const usize_t inc_x, f32_t *y, const usize_t inc_y);
void axpy_f64(const usize_t n, const f64_t alpha, const f64_t *x,
              const usize_t inc_x, f64_t *y, const usize_t inc_y);
void axpy_cf32(const usize_t n, const cf32_t *alpha, const cf32_t *x,
               const usize_t inc_x, cf32_t *y, const usize_t inc_y);
void axpy_cf64(const usize_t n, const cf64_t *alpha, const cf64_t *x,
               const usize_t inc_x, cf64_t *y, const usize_t inc_y);

// copy
void copy_f32(const usize_t n, const f32_t *x, const usize_t inc_x, f32_t *y,
              const usize_t inc_y);
void copy_f64(const usize_t n, const f64_t *x, const usize_t inc_x, f64_t *y,
              const usize_t inc_y);
void copy_cf32(const usize_t n, const cf32_t *x, const usize_t inc_x, cf32_t *y,
               const usize_t inc_y);
void copy_cf64(const usize_t n, const cf64_t *x, const usize_t inc_x, cf64_t *y,
               const usize_t inc_y);

// dot
f32_t dot_f32(const usize_t n, const f32_t *x, const usize_t inc_x,
              const f32_t *y, const usize_t inc_y);
f64_t dot_f32_as_f64(const usize_t n, const f32_t *x, const usize_t inc_x,
                     const f32_t *y, const usize_t inc_y);
f64_t dot_f64(const usize_t n, const f64_t *x, const usize_t inc_x,
              const f64_t *y, const usize_t inc_y);
cf32_t dot_cf32(const usize_t n, const cf32_t *x, const usize_t inc_x,
                const cf32_t *y, const usize_t inc_y);
cf32_t dot_cf32_conj(const usize_t n, const cf32_t *x, const usize_t inc_x,
                     const cf32_t *y, const usize_t inc_y);
cf64_t dot_cf64(const usize_t n, const cf64_t *x, const usize_t inc_x,
                const cf64_t *y, const usize_t inc_y);
cf64_t dot_cf64_conj(const usize_t n, const cf64_t *x, const usize_t inc_x,
                     const cf64_t *y, const usize_t inc_y);

// iamax
usize_t iamax_f32(const usize_t n, const f32_t *x, const usize_t inc_x);
usize_t iamax_f64(const usize_t n, const f64_t *x, const usize_t inc_x);
usize_t iamax_cf32(const usize_t n, const cf32_t *x, const usize_t inc_x);
usize_t iamax_cf64(const usize_t n, const cf64_t *x, const usize_t inc_x);

// rot
void rot_f32(const usize_t n, f32_t *x, const usize_t inc_x, f32_t *y,
             const usize_t inc_y, const f32_t c, const f32_t s);
void rot_f64(const usize_t n, f64_t *x, const usize_t inc_x, f64_t *y,
             const usize_t inc_y, const f64_t c, const f64_t s);
void rot_cf32(const usize_t n, cf32_t *x, const usize_t inc_x, cf32_t *y,
              const usize_t inc_y, const f32_t c, const f32_t s);
void rot_cf64(const usize_t n, cf64_t *x, const usize_t inc_x, cf64_t *y,
              const usize_t inc_y, const f64_t c, const f64_t s);

// scal
void scal_f32(const usize_t n, const f32_t alpha, f32_t *x,
              const usize_t inc_x);
void scal_f64(const usize_t n, const f64_t alpha, f64_t *x,
              const usize_t inc_x);
void scal_cf32(const usize_t n, const cf32_t *alpha, cf32_t *x,
               const usize_t inc_x);
void scal_cf64(const usize_t n, const cf64_t *alpha, cf64_t *x,
               const usize_t inc_x);

// swap
void swap_f32(const usize_t n, f32_t *x, const usize_t inc_x, f32_t *y,
              const usize_t inc_y);
void swap_f64(const usize_t n, f64_t *x, const usize_t inc_x, f64_t *y,
              const usize_t inc_y);
void swap_cf32(const usize_t n, cf32_t *x, const usize_t inc_x, cf32_t *y,
               const usize_t inc_y);
void swap_cf64(const usize_t n, cf64_t *x, const usize_t inc_x, cf64_t *y,
               const usize_t inc_y);

#ifdef __cplusplus
}
#endif

#endif /* RUBLAS_H */
