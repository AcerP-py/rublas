#ifndef CBLAS_H
#define CBLAS_H

#ifdef __cplusplus
extern "C" {
#endif

#define CBLAS_INT int64_t

#define CBLAS_INDEX size_t

/*
 * Level 1 BLAS
 */

// axpy
void cblas_saxpy(const CBLAS_INT N, const float alpha, const float *X,
                 const CBLAS_INT incX, float *Y, const CBLAS_INT incY);
void cblas_daxpy(const CBLAS_INT N, const double alpha, const double *X,
                 const CBLAS_INT incX, double *Y, const CBLAS_INT incY);
void cblas_caxpy(const CBLAS_INT N, const void *alpha, const void *X,
                 const CBLAS_INT incX, void *Y, const CBLAS_INT incY);
void cblas_zaxpy(const CBLAS_INT N, const void *alpha, const void *X,
                 const CBLAS_INT incX, void *Y, const CBLAS_INT incY);

// scal
void cblas_sscal(const CBLAS_INT N, const float alpha, float *X,
                 const CBLAS_INT incX);
void cblas_dscal(const CBLAS_INT N, const double alpha, double *X,
                 const CBLAS_INT incX);
void cblas_cscal(const CBLAS_INT N, const void *alpha, void *X,
                 const CBLAS_INT incX);
void cblas_zscal(const CBLAS_INT N, const void *alpha, void *X,
                 const CBLAS_INT incX);

// copy
void cblas_scopy(const CBLAS_INT N, const float *X, const CBLAS_INT incX,
                 float *Y, const CBLAS_INT incY);
void cblas_dcopy(const CBLAS_INT N, const double *X, const CBLAS_INT incX,
                 double *Y, const CBLAS_INT incY);
void cblas_ccopy(const CBLAS_INT N, const void *X, const CBLAS_INT incX,
                 void *Y, const CBLAS_INT incY);
void cblas_zcopy(const CBLAS_INT N, const void *X, const CBLAS_INT incX,
                 void *Y, const CBLAS_INT incY);

// iamax
CBLAS_INDEX cblas_isamax(const CBLAS_INT N, const float *X,
                         const CBLAS_INT incX);
CBLAS_INDEX cblas_idamax(const CBLAS_INT N, const double *X,
                         const CBLAS_INT incX);
CBLAS_INDEX cblas_icamax(const CBLAS_INT N, const void *X,
                         const CBLAS_INT incX);
CBLAS_INDEX cblas_izamax(const CBLAS_INT N, const void *X,
                         const CBLAS_INT incX);

#ifdef __cplusplus
}
#endif

#endif /* CBLAS_H */
