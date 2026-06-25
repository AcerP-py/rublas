#include "assert.h"
#include "cblas.h"
#include "rublas.h"

int main() {

  // ####################################################
  // rublas
  // ####################################################

  usize_t rublas_inc;

  // copy_f32
  START_TEST("copy_f32");
  {
    f32_t x[10];
    f32_t y[10];
    f64_t exp[10];

    INIT_ARRAY(x, 10, 1.0f);

    // Test sequential elements
    INIT_ARRAY(y, 10, 2.0f);
    rublas_inc = 1;
    copy_f32(10, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, x, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 2.0f);
    INIT_ARRAY2(exp, 10, 1.0f, 2.0f);
    rublas_inc = 2;
    copy_f32(5, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // copy_f64
  START_TEST("copy_f64");
  {
    f64_t x[10];
    f64_t y[10];
    f64_t exp[10];

    INIT_ARRAY(x, 10, 1.0);

    // Test sequential elements
    INIT_ARRAY(y, 10, 2.0);
    rublas_inc = 1;
    copy_f64(10, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, x, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 2.0);
    INIT_ARRAY2(exp, 10, 1.0, 2.0);
    rublas_inc = 2;
    copy_f64(5, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // copy_cf32
  START_TEST("copy_cf32");
  {
    cf32_t x[10];
    cf32_t y[10];
    cf32_t exp[10];

    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);

    // Test sequential elements
    INIT_ARRAY(y, 10, 2.0f + 2.0f * I);
    rublas_inc = 1;
    copy_cf32(10, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, x, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 2.0f + 2.0f * I);
    INIT_ARRAY2(exp, 10, 1.0f + 1.0f * I, 2.0f + 2.0f * I);
    rublas_inc = 2;
    copy_cf32(5, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // copy_cf64
  START_TEST("copy_cf64");
  {
    cf64_t x[10];
    cf64_t y[10];
    cf64_t exp[10];

    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);

    // Test sequential elements
    INIT_ARRAY(y, 10, 2.0 + 2.0 * I);
    rublas_inc = 1;
    copy_cf64(10, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, x, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 2.0 + 2.0 * I);
    INIT_ARRAY2(exp, 10, 1.0 + 1.0 * I, 2.0 + 2.0 * I);
    rublas_inc = 2;
    copy_cf64(5, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // ####################################################
  // cblas
  // ####################################################

  CBLAS_INDEX cblas_inc;

  // cblas_scopy
  START_TEST("cblas_scopy");
  {
    float x[10];
    float y[10];
    float exp[10];

    INIT_ARRAY(x, 10, 1.0f);

    // Test sequential elements
    INIT_ARRAY(y, 10, 2.0f);
    cblas_inc = 1;
    cblas_scopy(10, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, x, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 2.0f);
    INIT_ARRAY2(exp, 10, 1.0f, 2.0f);
    cblas_inc = 2;
    cblas_scopy(5, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // cblas_dcopy
  START_TEST("cblas_dcopy");
  {
    double x[10];
    double y[10];
    double exp[10];

    INIT_ARRAY(x, 10, 1.0);

    // Test sequential elements
    INIT_ARRAY(y, 10, 2.0);
    cblas_inc = 1;
    cblas_dcopy(10, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, x, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 2.0);
    INIT_ARRAY2(exp, 10, 1.0, 2.0);
    cblas_inc = 2;
    cblas_dcopy(5, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // cblas_ccopy
  START_TEST("cblas_ccopy");
  {
    float complex x[10];
    float complex y[10];
    float complex exp[10];

    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);

    // Test sequential elements
    INIT_ARRAY(y, 10, 2.0f + 2.0f * I);
    cblas_inc = 1;
    cblas_ccopy(10, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, x, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 2.0f + 2.0f * I);
    INIT_ARRAY2(exp, 10, 1.0f + 1.0f * I, 2.0f + 2.0f * I);
    cblas_inc = 2;
    cblas_ccopy(5, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  // cblas_zcopy
  START_TEST("cblas_zcopy");
  {
    double complex x[10];
    double complex y[10];
    double complex exp[10];

    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);

    // Test sequential elements
    INIT_ARRAY(y, 10, 2.0 + 2.0 * I);
    cblas_inc = 1;
    cblas_zcopy(10, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, x, 10);

    // Test non-sequential elements
    INIT_ARRAY(y, 10, 2.0 + 2.0 * I);
    INIT_ARRAY2(exp, 10, 1.0 + 1.0 * I, 2.0 + 2.0 * I);
    cblas_inc = 2;
    cblas_zcopy(5, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ_ARRAY(y, exp, 10);
  }
  END_TEST();

  return 0;
}
