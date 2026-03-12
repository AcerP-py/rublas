#include "assert.h"
#include "cblas.h"
#include "rublas.h"

int main() {

  // ####################################################
  // rublas
  // ####################################################

  usize_t inc;

  // scal_f32
  START_TEST("scal_f32");

  f32_t x32[10];
  f32_t exp32[10];

  // Test sequential elements
  INIT_ARRAY(x32, 10, 1.0f);
  INIT_ARRAY(exp32, 10, 2.0f);
  inc = 1;
  scal_f32(10, 2.0f, x32, inc);
  ASSERT_EQ_ARRAY(x32, exp32, 10);

  // Test non-sequential elements
  INIT_ARRAY(x32, 10, 1.0f);
  INIT_ARRAY2(exp32, 10, 2.0f, 1.0f);
  inc = 2;
  scal_f32(5, 2.0f, x32, inc);
  ASSERT_EQ_ARRAY(x32, exp32, 10);

  END_TEST();

  // scal_f64
  START_TEST("scal_f64");

  f64_t x64[10];
  f64_t exp64[10];

  // Test sequential elements
  INIT_ARRAY(x64, 10, 1.0);
  INIT_ARRAY(exp64, 10, 2.0);
  inc = 1;
  scal_f64(10, 2.0, x64, inc);
  ASSERT_EQ_ARRAY(x64, exp64, 10);

  // Test non-sequential elements
  INIT_ARRAY(x64, 10, 1.0);
  INIT_ARRAY2(exp64, 10, 2.0, 1.0);
  inc = 2;
  scal_f64(5, 2.0, x64, inc);
  ASSERT_EQ_ARRAY(x64, exp64, 10);

  END_TEST();

  // scal_cf32
  START_TEST("scal_cf32");

  cf32_t cx32[10];
  cf32_t cexp32[10];
  cf32_t calpha32 = {2.0f + 2.0f * I};

  // Test sequential elements
  INIT_ARRAY(cx32, 10, 1.0f + 1.0f * I);
  INIT_ARRAY(cexp32, 10, 0.0f + 4.0f * I);
  inc = 1;
  scal_cf32(10, &calpha32, cx32, inc);
  ASSERT_EQ_ARRAY(cx32, cexp32, 10);

  // Test non-sequential elements
  INIT_ARRAY(cx32, 10, 1.0f + 1.0f * I);
  INIT_ARRAY2(cexp32, 10, 0.0f + 4.0f * I, 1.0f + 1.0f * I);
  inc = 2;
  scal_cf32(5, &calpha32, cx32, inc);
  ASSERT_EQ_ARRAY(cx32, cexp32, 10);

  END_TEST();

  // scal_cf64
  START_TEST("scal_cf64");

  cf64_t cx64[10];
  cf64_t cexp64[10];
  cf64_t calpha64 = {2.0 + 2.0 * I};

  // Test sequential elements
  INIT_ARRAY(cx64, 10, 1.0 + 1.0 * I);
  INIT_ARRAY(cexp64, 10, 0.0 + 4.0 * I);
  inc = 1;
  scal_cf64(10, &calpha64, cx64, inc);
  ASSERT_EQ_ARRAY(cx64, cexp64, 10);

  // Test non-sequential elements
  INIT_ARRAY(cx64, 10, 1.0 + 1.0 * I);
  INIT_ARRAY2(cexp64, 10, 0.0 + 4.0 * I, 1.0 + 1.0 * I);
  inc = 2;
  scal_cf64(5, &calpha64, cx64, inc);
  ASSERT_EQ_ARRAY(cx64, cexp64, 10);

  END_TEST();

  // ####################################################
  // cblas
  // ####################################################

  CBLAS_INDEX cblas_inc;

  // cblas_sscal
  START_TEST("cblas_sscal");

  float cblas_x32[10];
  float cblas_exp32[10];

  // Test sequential elements
  INIT_ARRAY(cblas_x32, 10, 1.0f);
  INIT_ARRAY(cblas_exp32, 10, 2.0f);
  cblas_inc = 1;
  cblas_sscal(10, 2.0f, cblas_x32, cblas_inc);
  ASSERT_EQ_ARRAY(cblas_x32, cblas_exp32, 10);

  // Test non-sequential elements
  INIT_ARRAY(cblas_x32, 10, 1.0f);
  INIT_ARRAY2(cblas_exp32, 10, 2.0f, 1.0f);
  cblas_inc = 2;
  cblas_sscal(5, 2.0f, cblas_x32, cblas_inc);
  ASSERT_EQ_ARRAY(cblas_x32, cblas_exp32, 10);

  END_TEST();

  // cblas_dscal
  START_TEST("cblas_dscal");

  double cblas_x64[10];
  double cblas_exp64[10];

  // Test sequential elements
  INIT_ARRAY(cblas_x64, 10, 1.0);
  INIT_ARRAY(cblas_exp64, 10, 2.0);
  cblas_inc = 1;
  cblas_dscal(10, 2.0, cblas_x64, cblas_inc);
  ASSERT_EQ_ARRAY(cblas_x64, cblas_exp64, 10);

  // Test non-sequential elements
  INIT_ARRAY(cblas_x64, 10, 1.0);
  INIT_ARRAY2(cblas_exp64, 10, 2.0, 1.0);
  cblas_inc = 2;
  cblas_dscal(5, 2.0, cblas_x64, cblas_inc);
  ASSERT_EQ_ARRAY(cblas_x64, cblas_exp64, 10);

  END_TEST();

  // cblas_cscal
  START_TEST("cblas_cscal");

  float complex cblas_cx32[10];
  float complex cblas_cexp32[10];
  float complex cblas_calpha32 = {2.0f + 2.0f * I};

  // Test sequential elements
  INIT_ARRAY(cblas_cx32, 10, 1.0f + 1.0f * I);
  INIT_ARRAY(cblas_cexp32, 10, 0.0f + 4.0f * I);
  cblas_inc = 1;
  cblas_cscal(10, &cblas_calpha32, cblas_cx32, cblas_inc);
  ASSERT_EQ_ARRAY(cblas_cx32, cblas_cexp32, 10);

  // Test non-sequential elements
  INIT_ARRAY(cblas_cx32, 10, 1.0f + 1.0f * I);
  INIT_ARRAY2(cblas_cexp32, 10, 0.0f + 4.0f * I, 1.0f + 1.0f * I);
  cblas_inc = 2;
  cblas_cscal(5, &cblas_calpha32, cblas_cx32, cblas_inc);
  ASSERT_EQ_ARRAY(cblas_cx32, cblas_cexp32, 10);

  END_TEST();

  // cblas_zscal
  START_TEST("cblas_zscal");

  double complex cblas_cx64[10];
  double complex cblas_cexp64[10];
  double complex cblas_calpha64 = {2.0 + 2.0 * I};

  // Test sequential elements
  INIT_ARRAY(cblas_cx64, 10, 1.0 + 1.0 * I);
  INIT_ARRAY(cblas_cexp64, 10, 0.0 + 4.0 * I);
  cblas_inc = 1;
  cblas_zscal(10, &cblas_calpha64, cblas_cx64, cblas_inc);
  ASSERT_EQ_ARRAY(cblas_cx64, cblas_cexp64, 10);

  // Test non-sequential elements
  INIT_ARRAY(cblas_cx64, 10, 1.0 + 1.0 * I);
  INIT_ARRAY2(cblas_cexp64, 10, 0.0 + 4.0 * I, 1.0 + 1.0 * I);
  cblas_inc = 2;
  cblas_zscal(5, &cblas_calpha64, cblas_cx64, cblas_inc);
  ASSERT_EQ_ARRAY(cblas_cx64, cblas_cexp64, 10);

  END_TEST();

  return 0;
}
