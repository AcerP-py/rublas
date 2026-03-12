#include "assert.h"
#include "cblas.h"
#include "rublas.h"

int main() {

  // ####################################################
  // rublas
  // ####################################################

  usize_t inc;

  // iamax_f32
  START_TEST("iamax_f32");

  f32_t x32[10] = {1.0f, 9.0f,  3.0f, 4.0f, 3.0f,
                   6.0f, 11.0f, 8.0f, 9.0f, 10.0f};

  // Test sequential elements
  inc = 1;
  usize_t exp32 = iamax_f32(10, x32, inc);
  ASSERT_EQ(exp32, 6);

  // Test non-sequential elements
  inc = 2;
  exp32 = iamax_f32(5, x32, inc);
  ASSERT_EQ(exp32, 3);

  END_TEST();

  // iamax_f64
  START_TEST("iamax_f64");

  f64_t x64[10] = {1.0, 9.0, 3.0, 4.0, 3.0, 6.0, 11.0, 8.0, 9.0, 10.0};

  // Test sequential elements
  inc = 1;
  usize_t exp64 = iamax_f64(10, x64, inc);
  ASSERT_EQ(exp64, 6);

  // Test non-sequential elements
  inc = 2;
  exp64 = iamax_f64(5, x64, inc);
  ASSERT_EQ(exp64, 3);

  END_TEST();

  // iamax_cf32
  START_TEST("iamax_cf32");

  cf32_t cx32[10] = {
      1.0f + 1.0f * I, 1.0f + 9.0f * I,  3.0f + 1.0f * I, 4.0f + 1.0f * I,
      3.0f + 1.0f * I, 6.0f + 1.0f * I,  2.0f + 1.0f * I, 8.0f + 3.0f * I,
      9.0f + 1.0f * I, 10.0f + 1.0f * I,
  };

  // Test sequential elements
  inc = 1;
  usize_t cexp32 = iamax_cf32(10, cx32, inc);
  ASSERT_EQ(cexp32, 7);

  // Test non-sequential elements
  inc = 2;
  cexp32 = iamax_cf32(5, cx32, inc);
  ASSERT_EQ(cexp32, 4);

  END_TEST();

  // iamax_cf64
  START_TEST("iamax_cf64");

  cf64_t cx64[10] = {
      1.0 + 1.0 * I, 1.0 + 9.0 * I,  3.0 + 1.0 * I, 4.0 + 1.0 * I,
      3.0 + 1.0 * I, 6.0 + 1.0 * I,  2.0 + 1.0 * I, 8.0 + 3.0 * I,
      9.0 + 1.0 * I, 10.0 + 1.0 * I,
  };

  // Test sequential elements
  inc = 1;
  usize_t cexp64 = iamax_cf64(10, cx64, inc);
  ASSERT_EQ(cexp64, 7);

  // Test non-sequential elements
  inc = 2;
  cexp64 = iamax_cf64(5, cx64, inc);
  ASSERT_EQ(cexp64, 4);

  END_TEST();

  // ####################################################
  // cblas
  // ####################################################

  CBLAS_INDEX cblas_inc;

  // cblas_isamax
  START_TEST("cblas_isamax");

  float cblas_x32[10] = {1.0f, 9.0f,  3.0f, 4.0f, 3.0f,
                         6.0f, 11.0f, 8.0f, 9.0f, 10.0f};

  // Test sequential elements
  cblas_inc = 1;
  CBLAS_INDEX cblas_exp32 = cblas_isamax(10, cblas_x32, cblas_inc);
  ASSERT_EQ(cblas_exp32, 6);

  // Test non-sequential elements
  cblas_inc = 2;
  cblas_exp32 = cblas_isamax(5, cblas_x32, cblas_inc);
  ASSERT_EQ(cblas_exp32, 3);

  END_TEST();

  // cblas_idamax
  START_TEST("cblas_idamax");

  double cblas_x64[10] = {1.0, 9.0, 3.0, 4.0, 3.0, 6.0, 11.0, 8.0, 9.0, 10.0};

  // Test sequential elements
  cblas_inc = 1;
  CBLAS_INDEX cblas_exp64 = cblas_idamax(10, cblas_x64, cblas_inc);
  ASSERT_EQ(cblas_exp64, 6);

  // Test non-sequential elements
  cblas_inc = 2;
  cblas_exp64 = cblas_idamax(5, cblas_x64, cblas_inc);
  ASSERT_EQ(cblas_exp64, 3);

  END_TEST();

  // cblas_icamax
  START_TEST("cblas_icamax");

  float complex cblas_cx32[10] = {
      1.0f + 1.0f * I, 1.0f + 9.0f * I,  3.0f + 1.0f * I, 4.0f + 1.0f * I,
      3.0f + 1.0f * I, 6.0f + 1.0f * I,  2.0f + 1.0f * I, 8.0f + 3.0f * I,
      9.0f + 1.0f * I, 10.0f + 1.0f * I,
  };

  // Test sequential elements
  cblas_inc = 1;
  CBLAS_INDEX cblas_cexp32 = cblas_icamax(10, cblas_cx32, cblas_inc);
  ASSERT_EQ(cblas_cexp32, 7);

  // Test non-sequential elements
  cblas_inc = 2;
  cblas_cexp32 = cblas_icamax(5, cblas_cx32, cblas_inc);
  ASSERT_EQ(cblas_cexp32, 4);

  END_TEST();

  // cblas_izamax
  START_TEST("cblas_izamax");

  double complex cblas_cx64[10] = {
      1.0 + 1.0 * I, 1.0 + 9.0 * I,  3.0 + 1.0 * I, 4.0 + 1.0 * I,
      3.0 + 1.0 * I, 6.0 + 1.0 * I,  2.0 + 1.0 * I, 8.0 + 3.0 * I,
      9.0 + 1.0 * I, 10.0 + 1.0 * I,
  };

  // Test sequential elements
  cblas_inc = 1;
  CBLAS_INDEX cblas_cexp64 = cblas_izamax(10, cblas_cx64, cblas_inc);
  ASSERT_EQ(cblas_cexp64, 7);

  // Test non-sequential elements
  cblas_inc = 2;
  cblas_cexp64 = cblas_izamax(5, cblas_cx64, cblas_inc);
  ASSERT_EQ(cblas_cexp64, 4);

  END_TEST();

  return 0;
}
