#include "assert.h"
#include "cblas.h"
#include "rublas.h"

int main() {

  // ####################################################
  // rublas
  // ####################################################

  usize_t rublas_inc;

  // rotm_f32
  START_TEST("rotm_f32");
  {
    f32_t x[10];
    f32_t y[10];
    f32_t exp_x[10];
    f32_t exp_y[10];
    f32_t params[4] = {1.0f, 1.0f, 1.0f, 1.0f};

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);
    INIT_ARRAY(exp_x, 10, 3.0f);
    INIT_ARRAY(exp_y, 10, 3.0f);
    rublas_inc = 1;
    rotm_f32(10, x, rublas_inc, y, rublas_inc, RUBLAS_ROTM_FLAG_GENERAL,
             params);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);
    INIT_ARRAY2(exp_x, 10, 3.0f, 1.0f);
    INIT_ARRAY2(exp_y, 10, 3.0f, 2.0f);
    rublas_inc = 2;
    rotm_f32(5, x, rublas_inc, y, rublas_inc, RUBLAS_ROTM_FLAG_GENERAL, params);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);
  }
  END_TEST();

  // rotm_f64
  START_TEST("rotm_f64");
  {
    f64_t x[10];
    f64_t y[10];
    f64_t exp_x[10];
    f64_t exp_y[10];
    f64_t params[4] = {1.0, 1.0, 1.0, 1.0};

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY(y, 10, 2.0);
    INIT_ARRAY(exp_x, 10, 3.0);
    INIT_ARRAY(exp_y, 10, 3.0);
    rublas_inc = 1;
    rotm_f64(10, x, rublas_inc, y, rublas_inc, RUBLAS_ROTM_FLAG_GENERAL,
             params);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY(y, 10, 2.0);
    INIT_ARRAY2(exp_x, 10, 3.0, 1.0);
    INIT_ARRAY2(exp_y, 10, 3.0, 2.0);
    rublas_inc = 2;
    rotm_f64(5, x, rublas_inc, y, rublas_inc, RUBLAS_ROTM_FLAG_GENERAL, params);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);
  }
  END_TEST();

  // ####################################################
  // cblas
  // ####################################################

  CBLAS_INDEX cblas_inc;

  // cblas_srotm
  START_TEST("cblas_srotm");
  {
    float x[10];
    float y[10];
    float exp_x[10];
    float exp_y[10];
    float params[5] = {-1.0f, 1.0f, 1.0f, 1.0f, 1.0f};

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);
    INIT_ARRAY(exp_x, 10, 3.0f);
    INIT_ARRAY(exp_y, 10, 3.0f);
    cblas_inc = 1;
    cblas_srotm(10, x, cblas_inc, y, cblas_inc, params);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);
    INIT_ARRAY2(exp_x, 10, 3.0f, 1.0f);
    INIT_ARRAY2(exp_y, 10, 3.0f, 2.0f);
    cblas_inc = 2;
    cblas_srotm(5, x, cblas_inc, y, cblas_inc, params);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);
  }
  END_TEST();

  // cblas_drotm
  START_TEST("cblas_drotm");
  {
    double x[10];
    double y[10];
    double exp_x[10];
    double exp_y[10];
    double params[5] = {-1.0, 1.0, 1.0, 1.0, 1.0};

    // Test sequential elements
    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY(y, 10, 2.0);
    INIT_ARRAY(exp_x, 10, 3.0);
    INIT_ARRAY(exp_y, 10, 3.0);
    cblas_inc = 1;
    cblas_drotm(10, x, cblas_inc, y, cblas_inc, params);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);

    // Test non-sequential elements
    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY(y, 10, 2.0);
    INIT_ARRAY2(exp_x, 10, 3.0, 1.0);
    INIT_ARRAY2(exp_y, 10, 3.0, 2.0);
    cblas_inc = 2;
    cblas_drotm(5, x, cblas_inc, y, cblas_inc, params);
    ASSERT_EQ_ARRAY(x, exp_x, 10);
    ASSERT_EQ_ARRAY(y, exp_y, 10);
  }
  END_TEST();

  return 0;
}
