#include "assert.h"
#include "cblas.h"
#include "rublas.h"

int main() {

  // ####################################################
  // rublas
  // ####################################################

  usize_t rublas_inc;

  // rot_f32
  START_TEST("rot_f32");
  {
    f32_t x[10];
    f32_t y[10];
    f32_t exp_x[10];
    f32_t exp_y[10];
    f32_t c = 0.0f;
    f32_t s = 1.0f;

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);
    INIT_ARRAY(exp_x, 10, 2.0f);
    INIT_ARRAY(exp_y, 10, -1.0f);
    rublas_inc = 1;
    rot_f32(10, x, rublas_inc, y, rublas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);
    INIT_ARRAY2(exp_x, 10, 2.0f, 1.0f);
    INIT_ARRAY2(exp_y, 10, -1.0f, 2.0f);
    rublas_inc = 2;
    rot_f32(5, x, rublas_inc, y, rublas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);
  }
  END_TEST();

  // rot_f64
  START_TEST("rot_f64");
  {
    f64_t x[10];
    f64_t y[10];
    f64_t exp_x[10];
    f64_t exp_y[10];
    f64_t c = 0.0;
    f64_t s = 1.0;

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY(y, 10, 2.0);
    INIT_ARRAY(exp_x, 10, 2.0);
    INIT_ARRAY(exp_y, 10, -1.0);
    rublas_inc = 1;
    rot_f64(10, x, rublas_inc, y, rublas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY(y, 10, 2.0);
    INIT_ARRAY2(exp_x, 10, 2.0, 1.0);
    INIT_ARRAY2(exp_y, 10, -1.0, 2.0);
    rublas_inc = 2;
    rot_f64(5, x, rublas_inc, y, rublas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);
  }
  END_TEST();

  // rot_cf32
  START_TEST("rot_cf32");
  {
    cf32_t x[10];
    cf32_t y[10];
    cf32_t exp_x[10];
    cf32_t exp_y[10];
    f32_t c = 0.0f;
    f32_t s = 1.0f;

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);
    INIT_ARRAY(y, 10, 2.0f + 2.0f * I);
    INIT_ARRAY(exp_x, 10, 2.0f + 2.0f * I);
    INIT_ARRAY(exp_y, 10, -1.0f - 1.0f * I);
    rublas_inc = 1;
    rot_cf32(10, x, rublas_inc, y, rublas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);
    INIT_ARRAY(y, 10, 2.0f + 2.0f * I);
    INIT_ARRAY2(exp_x, 10, 2.0f + 2.0f * I, 1.0f + 1.0f * I);
    INIT_ARRAY2(exp_y, 10, -1.0f - 1.0f * I, 2.0f + 2.0f * I);
    rublas_inc = 2;
    rot_cf32(5, x, rublas_inc, y, rublas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);
  }
  END_TEST();

  // rot_cf64
  START_TEST("rot_cf64");
  {
    cf64_t x[10];
    cf64_t y[10];
    cf64_t exp_x[10];
    cf64_t exp_y[10];
    f64_t c = 0.0;
    f64_t s = 1.0;

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);
    INIT_ARRAY(y, 10, 2.0 + 2.0 * I);
    INIT_ARRAY(exp_x, 10, 2.0 + 2.0 * I);
    INIT_ARRAY(exp_y, 10, -1.0 - 1.0 * I);
    rublas_inc = 1;
    rot_cf64(10, x, rublas_inc, y, rublas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);
    INIT_ARRAY(y, 10, 2.0 + 2.0 * I);
    INIT_ARRAY2(exp_x, 10, 2.0 + 2.0 * I, 1.0 + 1.0 * I);
    INIT_ARRAY2(exp_y, 10, -1.0 - 1.0 * I, 2.0 + 2.0 * I);
    rublas_inc = 2;
    rot_cf64(5, x, rublas_inc, y, rublas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);
  }
  END_TEST();

  // ####################################################
  // cblas
  // ####################################################

  CBLAS_INDEX cblas_inc;

  // cblas_srot
  START_TEST("cblas_srot");
  {
    float x[10];
    float y[10];
    float exp_x[10];
    float exp_y[10];
    float c = 0.0f;
    float s = 1.0f;

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);
    INIT_ARRAY(exp_x, 10, 2.0f);
    INIT_ARRAY(exp_y, 10, -1.0f);
    cblas_inc = 1;
    cblas_srot(10, x, cblas_inc, y, cblas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);
    INIT_ARRAY2(exp_x, 10, 2.0f, 1.0f);
    INIT_ARRAY2(exp_y, 10, -1.0f, 2.0f);
    cblas_inc = 2;
    cblas_srot(5, x, cblas_inc, y, cblas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);
  }
  END_TEST();

  // cblas_drot
  START_TEST("cblas_drot");
  {
    double x[10];
    double y[10];
    double exp_x[10];
    double exp_y[10];
    double c = 0.0;
    double s = 1.0;

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY(y, 10, 2.0);
    INIT_ARRAY(exp_x, 10, 2.0);
    INIT_ARRAY(exp_y, 10, -1.0);
    cblas_inc = 1;
    cblas_drot(10, x, cblas_inc, y, cblas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY(y, 10, 2.0);
    INIT_ARRAY2(exp_x, 10, 2.0, 1.0);
    INIT_ARRAY2(exp_y, 10, -1.0, 2.0);
    cblas_inc = 2;
    cblas_drot(5, x, cblas_inc, y, cblas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);
  }
  END_TEST();

  // cblas_csrot
  START_TEST("cblas_csrot");
  {
    float complex x[10];
    float complex y[10];
    float complex exp_x[10];
    float complex exp_y[10];
    float c = 0.0f;
    float s = 1.0f;

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);
    INIT_ARRAY(y, 10, 2.0f + 2.0f * I);
    INIT_ARRAY(exp_x, 10, 2.0f + 2.0f * I);
    INIT_ARRAY(exp_y, 10, -1.0f - 1.0f * I);
    cblas_inc = 1;
    cblas_csrot(10, x, cblas_inc, y, cblas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);
    INIT_ARRAY(y, 10, 2.0f + 2.0f * I);
    INIT_ARRAY2(exp_x, 10, 2.0f + 2.0f * I, 1.0f + 1.0f * I);
    INIT_ARRAY2(exp_y, 10, -1.0f - 1.0f * I, 2.0f + 2.0f * I);
    cblas_inc = 2;
    cblas_csrot(5, x, cblas_inc, y, cblas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);
  }
  END_TEST();

  // cblas_zdrot
  START_TEST("cblas_zdrot");
  {
    double complex x[10];
    double complex y[10];
    double complex exp_x[10];
    double complex exp_y[10];
    double c = 0.0;
    double s = 1.0;

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);
    INIT_ARRAY(y, 10, 2.0 + 2.0 * I);
    INIT_ARRAY(exp_x, 10, 2.0 + 2.0 * I);
    INIT_ARRAY(exp_y, 10, -1.0 - 1.0 * I);
    cblas_inc = 1;
    cblas_zdrot(10, x, cblas_inc, y, cblas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);
    INIT_ARRAY(y, 10, 2.0 + 2.0 * I);
    INIT_ARRAY2(exp_x, 10, 2.0 + 2.0 * I, 1.0 + 1.0 * I);
    INIT_ARRAY2(exp_y, 10, -1.0 - 1.0 * I, 2.0 + 2.0 * I);
    cblas_inc = 2;
    cblas_zdrot(5, x, cblas_inc, y, cblas_inc, c, s);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);
  }
  END_TEST();

  return 0;
}
