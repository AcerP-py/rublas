#ifndef CBLAS_H
#define CBLAS_H

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Level 1 BLAS
 */

// axpy
void cblas_saxpy(const int N, const float alpha, const float *X, const int incX,
                 float *Y, const int incY);
void cblas_daxpy(const int N, const double alpha, const double *X,
                 const int incX, double *Y, const int incY);
void cblas_caxpy(const int N, const void *alpha, const void *X, const int incX,
                 void *Y, const int incY);
void cblas_zaxpy(const int N, const void *alpha, const void *X, const int incX,
                 void *Y, const int incY);

// scal
void cblas_sscal(const int N, const float alpha, float *X, const int incX);
void cblas_dscal(const int N, const double alpha, double *X, const int incX);
void cblas_cscal(const int N, const void *alpha, void *X, const int incX);
void cblas_zscal(const int N, const void *alpha, void *X, const int incX);

#ifdef __cplusplus
}
#endif

#endif /* CBLAS_H */
