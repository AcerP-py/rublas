#ifndef CBLAS_H
#define CBLAS_H

#include "rublas.h"

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Level 1 BLAS
 */

// axpy
#define cblas_saxpy _extern_c_saxpy
#define cblas_daxpy _extern_d_saxpy
#define cblas_caxpy _extern_c_saxpy
#define cblas_zaxpy _extern_z_saxpy

// scal
#define cblas_sscal _extern_c_sscal
#define cblas_dscal _extern_c_dscal

// dot
#define cblas_sdot _extern_c_sdot
#define cblas_ddot _extern_c_ddot

#ifdef __cplusplus
}
#endif

#endif /* CBLAS_H */
