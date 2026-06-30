#ifndef CBLAS_H
#define CBLAS_H
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define CBLAS_INT int64_t

#define CBLAS_INDEX size_t

/*
 * Level 1 BLAS
 */

// asum
float cblas_sasum(const CBLAS_INT N, const float *X, const CBLAS_INT incX);
double cblas_dasum(const CBLAS_INT N, const double *X, const CBLAS_INT incX);
float cblas_scasum(const CBLAS_INT N, const void *X, const CBLAS_INT incX);
double cblas_dzasum(const CBLAS_INT N, const void *X, const CBLAS_INT incX);

// axpy
void cblas_saxpy(const CBLAS_INT N, const float alpha, const float *X,
                 const CBLAS_INT incX, float *Y, const CBLAS_INT incY);
void cblas_daxpy(const CBLAS_INT N, const double alpha, const double *X,
                 const CBLAS_INT incX, double *Y, const CBLAS_INT incY);
void cblas_caxpy(const CBLAS_INT N, const void *alpha, const void *X,
                 const CBLAS_INT incX, void *Y, const CBLAS_INT incY);
void cblas_zaxpy(const CBLAS_INT N, const void *alpha, const void *X,
                 const CBLAS_INT incX, void *Y, const CBLAS_INT incY);

// copy
void cblas_scopy(const CBLAS_INT N, const float *X, const CBLAS_INT incX,
                 float *Y, const CBLAS_INT incY);
void cblas_dcopy(const CBLAS_INT N, const double *X, const CBLAS_INT incX,
                 double *Y, const CBLAS_INT incY);
void cblas_ccopy(const CBLAS_INT N, const void *X, const CBLAS_INT incX,
                 void *Y, const CBLAS_INT incY);
void cblas_zcopy(const CBLAS_INT N, const void *X, const CBLAS_INT incX,
                 void *Y, const CBLAS_INT incY);

// dot
float cblas_sdot(const CBLAS_INT N, const float *X, const CBLAS_INT incX,
                 const float *Y, const CBLAS_INT incY);
float cblas_sdsdot(const CBLAS_INT N, const float alpha, const float *X,
                   const CBLAS_INT incX, const float *Y, const CBLAS_INT incY);
double cblas_dsdot(const CBLAS_INT N, const float *X, const CBLAS_INT incX,
                   const float *Y, const CBLAS_INT incY);
double cblas_ddot(const CBLAS_INT N, const double *X, const CBLAS_INT incX,
                  const double *Y, const CBLAS_INT incY);
void cblas_cdotu_sub(const CBLAS_INT N, const void *X, const CBLAS_INT incX,
                     const void *Y, const CBLAS_INT incY, void *dotu);
void cblas_cdotc_sub(const CBLAS_INT N, const void *X, const CBLAS_INT incX,
                     const void *Y, const CBLAS_INT incY, void *dotc);
void cblas_zdotu_sub(const CBLAS_INT N, const void *X, const CBLAS_INT incX,
                     const void *Y, const CBLAS_INT incY, void *dotu);
void cblas_zdotc_sub(const CBLAS_INT N, const void *X, const CBLAS_INT incX,
                     const void *Y, const CBLAS_INT incY, void *dotc);

// iamax
CBLAS_INDEX cblas_isamax(const CBLAS_INT N, const float *X,
                         const CBLAS_INT incX);
CBLAS_INDEX cblas_idamax(const CBLAS_INT N, const double *X,
                         const CBLAS_INT incX);
CBLAS_INDEX cblas_icamax(const CBLAS_INT N, const void *X,
                         const CBLAS_INT incX);
CBLAS_INDEX cblas_izamax(const CBLAS_INT N, const void *X,
                         const CBLAS_INT incX);

// rot
void cblas_srot(const CBLAS_INT N, float *X, const CBLAS_INT incX, float *Y,
                const CBLAS_INT incY, const float c, const float s);
void cblas_drot(const CBLAS_INT N, double *X, const CBLAS_INT incX, double *Y,
                const CBLAS_INT incY, const double c, const double s);
void cblas_csrot(const CBLAS_INT N, void *X, const CBLAS_INT incX, void *Y,
                 const CBLAS_INT incY, const float c, const float s);
void cblas_zdrot(const CBLAS_INT N, void *X, const CBLAS_INT incX, void *Y,
                 const CBLAS_INT incY, const double c, const double s);

// scal
void cblas_sscal(const CBLAS_INT N, const float alpha, float *X,
                 const CBLAS_INT incX);
void cblas_dscal(const CBLAS_INT N, const double alpha, double *X,
                 const CBLAS_INT incX);
void cblas_cscal(const CBLAS_INT N, const void *alpha, void *X,
                 const CBLAS_INT incX);
void cblas_zscal(const CBLAS_INT N, const void *alpha, void *X,
                 const CBLAS_INT incX);

// swap
void cblas_sswap(const CBLAS_INT N, float *X, const CBLAS_INT incX, float *Y,
                 const CBLAS_INT incY);
void cblas_dswap(const CBLAS_INT N, double *X, const CBLAS_INT incX, double *Y,
                 const CBLAS_INT incY);
void cblas_cswap(const CBLAS_INT N, void *X, const CBLAS_INT incX, void *Y,
                 const CBLAS_INT incY);
void cblas_zswap(const CBLAS_INT N, void *X, const CBLAS_INT incX, void *Y,
                 const CBLAS_INT incY);

#ifdef __cplusplus
}
#endif

#endif /* CBLAS_H */
