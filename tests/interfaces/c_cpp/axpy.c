#include "assert.h"
#include "cblas.h"
#include "rublas.h"

int main() {

  // ####################################################
  // rublas
  // ####################################################

  usize_t rublas_inc;

  // axpy_f32
  START_TEST("axpy_f32");
  {
    f32_t x[10];
    f32_t y[10];
    f32_t alpha = 2.0f;
    f32_t exp[10];

    INIT_ARRAY(x, 10, 1.0f);

    // Test sequential elements
    INIT_ARRAY(y, 10, 1.0f);
    INIT_ARRAY(exp, 10, 3.0f);
    rublas_inc = 1;
    axpy_f32(10, alpha, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 1.0f);
    INIT_ARRAY2(exp, 10, 3.0f, 1.0f);
    rublas_inc = 2;
    axpy_f32(5, alpha, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // axpy_f64
  START_TEST("axpy_f64");
  {
    f64_t x[10];
    f64_t y[10];
    f64_t alpha = 2.0;
    f64_t exp[10];

    INIT_ARRAY(x, 10, 1.0);

    // Test sequential elements
    INIT_ARRAY(y, 10, 1.0);
    INIT_ARRAY(exp, 10, 3.0);
    rublas_inc = 1;
    axpy_f64(10, alpha, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 1.0);
    INIT_ARRAY2(exp, 10, 3.0, 1.0);
    rublas_inc = 2;
    axpy_f64(5, alpha, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // axpy_cf32
  START_TEST("axpy_cf32");
  {
    cf32_t x[10];
    cf32_t y[10];
    cf32_t alpha = {2.0f + 2.0f * I};
    cf32_t exp[10];

    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);

    // Test sequential elements
    INIT_ARRAY(y, 10, 1.0f + 1.0f * I);
    INIT_ARRAY(exp, 10, 1.0f + 5.0f * I);
    rublas_inc = 1;
    axpy_cf32(10, &alpha, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 1.0f + 1.0f * I);
    INIT_ARRAY2(exp, 10, 1.0f + 5.0f * I, 1.0f + 1.0f * I);
    rublas_inc = 2;
    axpy_cf32(5, &alpha, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // axpy_cf64
  START_TEST("axpy_cf64");
  {
    cf64_t x[10];
    cf64_t y[10];
    cf64_t alpha = {2.0 + 2.0 * I};
    cf64_t exp[10];

    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);

    // Test sequential elements
    INIT_ARRAY(y, 10, 1.0 + 1.0 * I);
    INIT_ARRAY(exp, 10, 1.0 + 5.0 * I);
    rublas_inc = 1;
    axpy_cf64(10, &alpha, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 1.0 + 1.0 * I);
    INIT_ARRAY2(exp, 10, 1.0 + 5.0 * I, 1.0 + 1.0 * I);
    rublas_inc = 2;
    axpy_cf64(5, &alpha, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // ####################################################
  // cblas
  // ####################################################

  CBLAS_INDEX cblas_inc;

  // cblas_saxpy
  START_TEST("cblas_saxpy");
  {
    float x[10];
    float y[10];
    float alpha = 2.0f;
    float exp[10];

    INIT_ARRAY(x, 10, 1.0f);

    // Test sequential elements
    INIT_ARRAY(y, 10, 1.0f);
    INIT_ARRAY(exp, 10, 3.0f);
    cblas_inc = 1;
    cblas_saxpy(10, alpha, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 1.0f);
    INIT_ARRAY2(exp, 10, 3.0f, 1.0f);
    cblas_inc = 2;
    cblas_saxpy(5, alpha, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // cblas_daxpy
  START_TEST("cblas_daxpy");
  {
    double x[10];
    double y[10];
    double alpha = 2.0;
    double exp[10];

    INIT_ARRAY(x, 10, 1.0);

    // Test sequential elements
    INIT_ARRAY(y, 10, 1.0);
    INIT_ARRAY(exp, 10, 3.0);
    cblas_inc = 1;
    cblas_daxpy(10, alpha, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 1.0);
    INIT_ARRAY2(exp, 10, 3.0, 1.0);
    cblas_inc = 2;
    cblas_daxpy(5, alpha, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // cblas_caxpy
  START_TEST("cblas_caxpy");
  {
    float complex x[10];
    float complex y[10];
    float complex alpha = {2.0f + 2.0f * I};
    float complex exp[10];

    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);

    // Test sequential elements
    INIT_ARRAY(y, 10, 1.0f + 1.0f * I);
    INIT_ARRAY(exp, 10, 1.0f + 5.0f * I);
    cblas_inc = 1;
    cblas_caxpy(10, &alpha, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 1.0f + 1.0f * I);
    INIT_ARRAY2(exp, 10, 1.0f + 5.0f * I, 1.0f + 1.0f * I);
    cblas_inc = 2;
    cblas_caxpy(5, &alpha, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // cblas_zaxpy
  START_TEST("cblas_zaxpy");
  {
    double complex x[10];
    double complex y[10];
    double complex alpha = {2.0 + 2.0 * I};
    double complex exp[10];

    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);

    // Test sequential elements
    INIT_ARRAY(y, 10, 1.0 + 1.0 * I);
    INIT_ARRAY(exp, 10, 1.0 + 5.0 * I);
    cblas_inc = 1;
    cblas_zaxpy(10, &alpha, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 1.0 + 1.0 * I);
    INIT_ARRAY2(exp, 10, 1.0 + 5.0 * I, 1.0 + 1.0 * I);
    cblas_inc = 2;
    cblas_zaxpy(5, &alpha, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  return 0;
}
