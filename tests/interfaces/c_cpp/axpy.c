#include "assert.h"
#include "cblas.h"
#include "rublas.h"

int main() {

  // ####################################################
  // rublas
  // ####################################################

  // axpy_f32
  START_TEST("axpy_f32");

  f32_t x32[10];
  f32_t y32[10];
  f32_t exp32[10];

  INIT_ARRAY(x32, 10, 1.0f);

  // Test sequential elements
  INIT_ARRAY(y32, 10, 1.0f);
  INIT_ARRAY(exp32, 10, 3.0f);
  axpy_f32(10, 2.0f, x32, 1, y32, 1);
  ASSERT_EQ_ARRAY(y32, exp32, 10);

  // Test non-sequential elements
  INIT_ARRAY(y32, 10, 1.0f);
  INIT_ARRAY2(exp32, 10, 3.0f, 1.0f);
  axpy_f32(5, 2.0f, x32, 2, y32, 2);
  ASSERT_EQ_ARRAY(y32, exp32, 10);

  END_TEST();

  // axpy_f64
  START_TEST("axpy_f64");

  f64_t x64[10];
  f64_t y64[10];
  f64_t exp64[10];

  INIT_ARRAY(x64, 10, 1.0);

  // Test sequential elements
  INIT_ARRAY(y64, 10, 1.0);
  INIT_ARRAY(exp64, 10, 3.0);
  axpy_f64(10, 2.0, x64, 1, y64, 1);
  ASSERT_EQ_ARRAY(y64, exp64, 10);

  // Test non-sequential elements
  INIT_ARRAY(y64, 10, 1.0);
  INIT_ARRAY2(exp64, 10, 3.0, 1.0);
  axpy_f64(5, 2.0, x64, 2, y64, 2);
  ASSERT_EQ_ARRAY(y64, exp64, 10);

  END_TEST();

  // axpy_cf32
  START_TEST("axpy_cf32");

  cf32_t cx32[10];
  cf32_t cy32[10];
  cf32_t cexp32[10];
  cf32_t calpha32 = {2.0f + 2.0f * I};

  INIT_ARRAY(cx32, 10, 1.0f + 1.0f * I);

  // Test sequential elements
  INIT_ARRAY(cy32, 10, 1.0f + 1.0f * I);
  INIT_ARRAY(cexp32, 10, 1.0f + 5.0f * I);
  axpy_cf32(10, &calpha32, cx32, 1, cy32, 1);
  ASSERT_EQ_ARRAY(cy32, cexp32, 10);

  // Test non-sequential elements
  INIT_ARRAY(cy32, 10, 1.0f + 1.0f * I);
  INIT_ARRAY2(cexp32, 10, 1.0f + 5.0f * I, 1.0f + 1.0f * I);
  axpy_cf32(5, &calpha32, cx32, 2, cy32, 2);
  ASSERT_EQ_ARRAY(cy32, cexp32, 10);

  END_TEST();

  // axpy_cf64
  START_TEST("axpy_cf64");

  cf64_t cx64[10];
  cf64_t cy64[10];
  cf64_t cexp64[10];
  cf64_t calpha64 = {2.0 + 2.0 * I};

  INIT_ARRAY(cx64, 10, 1.0 + 1.0 * I);

  // Test sequential elements
  INIT_ARRAY(cy64, 10, 1.0 + 1.0 * I);
  INIT_ARRAY(cexp64, 10, 1.0 + 5.0 * I);
  axpy_cf64(10, &calpha64, cx64, 1, cy64, 1);
  ASSERT_EQ_ARRAY(cy64, cexp64, 10);

  // Test non-sequential elements
  INIT_ARRAY(cy64, 10, 1.0 + 1.0 * I);
  INIT_ARRAY2(cexp64, 10, 1.0 + 5.0 * I, 1.0 + 1.0 * I);
  axpy_cf64(5, &calpha64, cx64, 2, cy64, 2);
  ASSERT_EQ_ARRAY(cy64, cexp64, 10);

  END_TEST();

  // ####################################################
  // cblas
  // ####################################################

  // cblas_saxpy
  START_TEST("cblas_saxpy");

  float cblas_x32[10];
  float cblas_y32[10];
  float cblas_exp32[10];

  INIT_ARRAY(cblas_x32, 10, 1.0f);

  // Test sequential elements
  INIT_ARRAY(cblas_y32, 10, 1.0f);
  INIT_ARRAY(cblas_exp32, 10, 3.0f);
  cblas_saxpy(10, 2.0f, cblas_x32, 1, cblas_y32, 1);
  ASSERT_EQ_ARRAY(cblas_y32, cblas_exp32, 10);

  // Test non-sequential elements
  INIT_ARRAY(cblas_y32, 10, 1.0f);
  INIT_ARRAY2(cblas_exp32, 10, 3.0f, 1.0f);
  cblas_saxpy(5, 2.0f, cblas_x32, 2, cblas_y32, 2);
  ASSERT_EQ_ARRAY(cblas_y32, cblas_exp32, 10);

  END_TEST();

  // cblas_daxpy
  START_TEST("cblas_daxpy");

  double cblas_x64[10];
  double cblas_y64[10];
  double cblas_exp64[10];

  INIT_ARRAY(cblas_x64, 10, 1.0);

  // Test sequential elements
  INIT_ARRAY(cblas_y64, 10, 1.0);
  INIT_ARRAY(cblas_exp64, 10, 3.0);
  cblas_daxpy(10, 2.0, cblas_x64, 1, cblas_y64, 1);
  ASSERT_EQ_ARRAY(cblas_y64, cblas_exp64, 10);

  // Test non-sequential elements
  INIT_ARRAY(cblas_y64, 10, 1.0);
  INIT_ARRAY2(cblas_exp64, 10, 3.0, 1.0);
  cblas_daxpy(5, 2.0, cblas_x64, 2, cblas_y64, 2);
  ASSERT_EQ_ARRAY(cblas_y64, cblas_exp64, 10);

  END_TEST();

  // cblas_caxpy
  START_TEST("cblas_caxpy");

  float complex cblas_cx32[10];
  float complex cblas_cy32[10];
  float complex cblas_cexp32[10];
  float complex cblas_calpha32 = {2.0f + 2.0f * I};

  INIT_ARRAY(cblas_cx32, 10, 1.0f + 1.0f * I);

  // Test sequential elements
  INIT_ARRAY(cblas_cy32, 10, 1.0f + 1.0f * I);
  INIT_ARRAY(cblas_cexp32, 10, 1.0f + 5.0f * I);
  cblas_caxpy(10, &cblas_calpha32, cblas_cx32, 1, cblas_cy32, 1);
  ASSERT_EQ_ARRAY(cblas_cy32, cblas_cexp32, 10);

  // Test non-sequential elements
  INIT_ARRAY(cblas_cy32, 10, 1.0f + 1.0f * I);
  INIT_ARRAY2(cblas_cexp32, 10, 1.0f + 5.0f * I, 1.0f + 1.0f * I);
  cblas_caxpy(5, &cblas_calpha32, cblas_cx32, 2, cblas_cy32, 2);
  ASSERT_EQ_ARRAY(cblas_cy32, cblas_cexp32, 10);

  END_TEST();

  // cblas_zaxpy
  START_TEST("cblas_zaxpy");

  double complex cblas_cx64[10];
  double complex cblas_cy64[10];
  double complex cblas_cexp64[10];
  double complex cblas_calpha64 = {2.0 + 2.0 * I};

  INIT_ARRAY(cblas_cx64, 10, 1.0 + 1.0 * I);

  // Test sequential elements
  INIT_ARRAY(cblas_cy64, 10, 1.0 + 1.0 * I);
  INIT_ARRAY(cblas_cexp64, 10, 1.0 + 5.0 * I);
  cblas_zaxpy(10, &cblas_calpha64, cblas_cx64, 1, cblas_cy64, 1);
  ASSERT_EQ_ARRAY(cblas_cy64, cblas_cexp64, 10);

  // Test non-sequential elements
  INIT_ARRAY(cblas_cy64, 10, 1.0 + 1.0 * I);
  INIT_ARRAY2(cblas_cexp64, 10, 1.0 + 5.0 * I, 1.0 + 1.0 * I);
  cblas_zaxpy(5, &cblas_calpha64, cblas_cx64, 2, cblas_cy64, 2);
  ASSERT_EQ_ARRAY(cblas_cy64, cblas_cexp64, 10);

  END_TEST();

  return 0;
}
