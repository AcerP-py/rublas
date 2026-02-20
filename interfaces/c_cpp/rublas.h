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

typedef size_t rublas_uint_t;

typedef float Float32_t;

typedef double Float64_t;

typedef float complex CFloat32_t;

typedef double complex CFloat64_t;

/*
 * Level 1 BLAS
 */

// axpy
void axpy_float32(const rublas_uint_t n, const Float32_t alpha,
                  const Float32_t *x, const rublas_uint_t inc_x, Float32_t *y,
                  const rublas_uint_t inc_y);
void axpy_float64(const rublas_uint_t n, const Float64_t alpha,
                  const Float64_t *x, const rublas_uint_t inc_x, Float64_t *y,
                  const rublas_uint_t inc_y);
void axpy_cfloat32(const rublas_uint_t n, const CFloat32_t *alpha,
                   const CFloat32_t *x, const rublas_uint_t inc_x,
                   CFloat32_t *y, const rublas_uint_t inc_y);
void axpy_cfloat64(const rublas_uint_t n, const CFloat64_t *alpha,
                   const CFloat64_t *x, const rublas_uint_t inc_x,
                   CFloat64_t *y, const rublas_uint_t inc_y);

// scal
void scal_float32(const rublas_uint_t n, const Float32_t alpha, Float32_t *x,
                  const rublas_uint_t inc_x);
void scal_float64(const rublas_uint_t n, const Float64_t alpha, Float64_t *x,
                  const rublas_uint_t inc_x);
void scal_cfloat32(const rublas_uint_t n, const CFloat32_t *alpha,
                   CFloat32_t *x, const rublas_uint_t inc_x);
void scal_cfloat64(const rublas_uint_t n, const CFloat64_t *alpha,
                   CFloat64_t *x, const rublas_uint_t inc_x);

#ifdef __cplusplus
}
#endif

#endif /* RUBLAS_H */
