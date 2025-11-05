#ifndef RUBLAS_H
#define RUBLAS_H

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Level 1 BLAS
 */

// axpy
void _extern_c_saxpy(const int n, const float alpha, const float *x,
                     const int inc_x, float *y, const int inc_y);
#define saxpy _extern_c_saxpy

void _extern_c_daxpy(const int n, const double alpha, const double *x,
                     const int inc_x, double *y, const int inc_y);
#define daxpy _extern_c_daxpy

void _extern_c_caxpy(const int n, const void *alpha, const void *x,
                     const int inc_x, void *y, const int inc_y);
#define caxpy _extern_c_caxpy

void _extern_c_zaxpy(const int n, const void *alpha, const void *x,
                     const int inc_x, void *y, const int inc_y);
#define zaxpy _extern_c_zaxpy

// scal
void _extern_c_sscal(const int n, const float alpha, float *x, const int inc_x);
#define sscal _extern_c_sscal

void _extern_c_dscal(const int n, const double alpha, double *x,
                     const int inc_x);
#define dscal _extern_c_dscal

// dot
float _extern_c_sdot(const int n, const float *x, const int inc_x,
                     const float *y, const int inc_y);
#define sdot _extern_c_sdot

double _extern_c_ddot(const int n, const double *x, const int inc_x,
                      const double *y, const int inc_y);
#define ddot _extern_c_ddot

#ifdef __cplusplus
}
#endif

#endif /* RUBLAS_H */
