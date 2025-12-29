#ifndef CBLAS_H
#define CBLAS_H
#include "rublas.h"

#ifdef __cplusplus
extern "C" {
#endif

/*
 * types
 */

#define CBLAS_INT RUBLAS_UINT

/*
 * Level 1 BLAS
 */

// axpy
#define cblas_saxpy(N, alpha, X, incX, Y, incY)                                \
  axpy_float32((RUBLAS_UINT)(N), (const Float32_t)(alpha),                     \
               (const Float32_t *)(X), (RUBLAS_UINT)(incX), (Float32_t *)(Y),  \
               (RUBLAS_UINT)(incY))

#define cblas_daxpy(N, alpha, X, incX, Y, incY)                                \
  axpy_float64((RUBLAS_UINT)(N), (const Float64_t)(alpha),                     \
               (const Float64_t *)(X), (RUBLAS_UINT)(incX), (Float64_t *)(Y),  \
               (RUBLAS_UINT)(incY))

#define cblas_caxpy(N, alpha, X, incX, Y, incY)                                \
  axpy_cfloat32((RUBLAS_UINT)(N), (const CFloat32_t *)(alpha),                 \
                (const CFloat32_t *)(X), (RUBLAS_UINT)(incX),                  \
                (CFloat32_t *)(Y), (RUBLAS_UINT)(incY))

#define cblas_zaxpy(N, alpha, X, incX, Y, incY)                                \
  axpy_cfloat64((RUBLAS_UINT)(N), (const CFloat64_t *)(alpha),                 \
                (const CFloat64_t *)(X), (RUBLAS_UINT)(incX),                  \
                (CFloat64_t *)(Y), (RUBLAS_UINT)(incY))

#ifdef __cplusplus
}
#endif

#endif /* CBLAS_H */
