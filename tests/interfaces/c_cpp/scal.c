#include "assert.h"
#include "cblas.h"
#include "rublas.h"

int main() {

  // ####################################################
  // rublas
  // ####################################################

  usize_t rublas_inc;

  // scal_f32
  START_TEST("scal_f32");
  {
    f32_t x[10];
    f32_t exp[10];

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(exp, 10, 2.0f);
    rublas_inc = 1;
    scal_f32(10, 2.0f, x, rublas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY2(exp, 10, 2.0f, 1.0f);
    rublas_inc = 2;
    scal_f32(5, 2.0f, x, rublas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);
  }
  END_TEST();

  // scal_f64
  START_TEST("scal_f64");
  {
    f64_t x[10];
    f64_t exp[10];

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY(exp, 10, 2.0);
    rublas_inc = 1;
    scal_f64(10, 2.0, x, rublas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY2(exp, 10, 2.0, 1.0);
    rublas_inc = 2;
    scal_f64(5, 2.0, x, rublas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);
  }
  END_TEST();

  // scal_cf32
  START_TEST("scal_cf32");
  {
    cf32_t x[10];
    cf32_t exp[10];
    cf32_t alpha = {2.0f + 2.0f * I};

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);
    INIT_ARRAY(exp, 10, 0.0f + 4.0f * I);
    rublas_inc = 1;
    scal_cf32(10, &alpha, x, rublas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);
    INIT_ARRAY2(exp, 10, 0.0f + 4.0f * I, 1.0f + 1.0f * I);
    rublas_inc = 2;
    scal_cf32(5, &alpha, x, rublas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);
  }
  END_TEST();

  // scal_cf64
  START_TEST("scal_cf64");
  {
    cf64_t x[10];
    cf64_t exp[10];
    cf64_t alpha = {2.0 + 2.0 * I};

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);
    INIT_ARRAY(exp, 10, 0.0 + 4.0 * I);
    rublas_inc = 1;
    scal_cf64(10, &alpha, x, rublas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);
    INIT_ARRAY2(exp, 10, 0.0 + 4.0 * I, 1.0 + 1.0 * I);
    rublas_inc = 2;
    scal_cf64(5, &alpha, x, rublas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);
  }
  END_TEST();

  // ####################################################
  // cblas
  // ####################################################

  CBLAS_INDEX cblas_inc;

  // cblas_sscal
  START_TEST("cblas_sscal");
  {
    float x[10];
    float exp[10];

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(exp, 10, 2.0f);
    cblas_inc = 1;
    cblas_sscal(10, 2.0f, x, cblas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY2(exp, 10, 2.0f, 1.0f);
    cblas_inc = 2;
    cblas_sscal(5, 2.0f, x, cblas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);
  }
  END_TEST();

  // cblas_dscal
  START_TEST("cblas_dscal");
  {
    double x[10];
    double exp[10];

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY(exp, 10, 2.0);
    cblas_inc = 1;
    cblas_dscal(10, 2.0, x, cblas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY2(exp, 10, 2.0, 1.0);
    cblas_inc = 2;
    cblas_dscal(5, 2.0, x, cblas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);
  }
  END_TEST();

  // cblas_cscal
  START_TEST("cblas_cscal");
  {
    float complex x[10];
    float complex exp[10];
    float complex alpha = {2.0f + 2.0f * I};

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);
    INIT_ARRAY(exp, 10, 0.0f + 4.0f * I);
    cblas_inc = 1;
    cblas_cscal(10, &alpha, x, cblas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);
    INIT_ARRAY2(exp, 10, 0.0f + 4.0f * I, 1.0f + 1.0f * I);
    cblas_inc = 2;
    cblas_cscal(5, &alpha, x, cblas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);
  }
  END_TEST();

  // cblas_zscal
  START_TEST("cblas_zscal");
  {
    double complex x[10];
    double complex exp[10];
    double complex alpha = {2.0 + 2.0 * I};

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);
    INIT_ARRAY(exp, 10, 0.0 + 4.0 * I);
    cblas_inc = 1;
    cblas_zscal(10, &alpha, x, cblas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);
    INIT_ARRAY2(exp, 10, 0.0 + 4.0 * I, 1.0 + 1.0 * I);
    cblas_inc = 2;
    cblas_zscal(5, &alpha, x, cblas_inc);
    ASSERT_EQ_ARRAY(x, exp, 10);
  }
  END_TEST();

  return 0;
}
