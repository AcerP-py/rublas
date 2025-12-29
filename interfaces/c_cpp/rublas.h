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

#define RUBLAS_UINT size_t

typedef float Float32_t;

typedef double Float64_t;

typedef float complex CFloat32_t;

typedef double complex CFloat64_t;

/*
 * Level 1 BLAS
 */

// axpy
void axpy_float32(const RUBLAS_UINT n, const Float32_t alpha,
                  const Float32_t *x, const RUBLAS_UINT inc_x, Float32_t *y,
                  const RUBLAS_UINT inc_y);

void axpy_float64(const RUBLAS_UINT n, const Float64_t alpha,
                  const Float64_t *x, const RUBLAS_UINT inc_x, Float64_t *y,
                  const RUBLAS_UINT inc_y);

void axpy_cfloat32(const RUBLAS_UINT n, const CFloat32_t *alpha,
                   const CFloat32_t *x, const RUBLAS_UINT inc_x, CFloat32_t *y,
                   const RUBLAS_UINT inc_y);

void axpy_cfloat64(const RUBLAS_UINT n, const CFloat64_t *alpha,
                   const CFloat64_t *x, const RUBLAS_UINT inc_x, CFloat64_t *y,
                   const RUBLAS_UINT inc_y);

#ifdef __cplusplus
}
#endif

#endif /* RUBLAS_H */
