#ifndef RUBLAS_H
#define RUBLAS_H

#ifdef __cplusplus
extern "C" {
#endif

// axpy
void rublas_saxpy(const int n, const float alpha, const float *x,
                 const int inc_x, float *y, const int inc_y);
#define cblas_saxpy rublas_saxpy

void rublas_daxpy(const int n, const double alpha, const double *x,
                 const int inc_x, double *y, const int inc_y);
#define cblas_daxpy rublas_daxpy

void rublas_caxpy(const int n, const void *alpha, const void *x,
                 const int inc_x, void *y, const int inc_y);
#define cblas_caxpy rublas_caxpy

void rublas_zaxpy(const int n, const void *alpha, const void *x,
                 const int inc_x, void *y, const int inc_y);
#define cblas_zaxpy rublas_zaxpy

// scal
void rublas_sscal(const int n, const float alpha, float *x, const int inc_x);
#define cblas_sscal rublas_sscal

// dot
float  rublas_sdot(const int n, const float  *x, const int inc_x,
                  const float  *y, const int inc_y);
#define cblas_sdot rublas_sdot

double rublas_ddot(const int n, const double *x, const int inc_x,
                  const double *y, const int inc_y);
#define cblas_ddot rublas_ddot

#ifdef __cplusplus
}
#endif

#endif /* RUBLAS_H */
