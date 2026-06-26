#include "assert.h"
#include "cblas.h"
#include "rublas.h"

int main() {

  // ####################################################
  // rublas
  // ####################################################

  usize_t rublas_inc;

  // asum_f32
  START_TEST("asum_f32");
  {
    f32_t x[10] = {1.0f,  -2.0f, 3.0f,  -4.0f, 5.0f,
                   -6.0f, 7.0f,  -8.0f, 9.0f,  -10.0f};
    f32_t res;

    // Test sequential elements
    rublas_inc = 1;
    res = asum_f32(10, x, rublas_inc);
    ASSERT_EQ(res, 55.0f);

    // Test non-sequential elements
    rublas_inc = 2;
    res = asum_f32(5, x, rublas_inc);
    ASSERT_EQ(res, 25.0f);
  }
  END_TEST();

  // asum_f64
  START_TEST("asum_f64");
  {
    f64_t x[10] = {1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0, 9.0, -10.0};
    f64_t res;

    // Test sequential elements
    rublas_inc = 1;
    res = asum_f64(10, x, rublas_inc);
    ASSERT_EQ(res, 55.0);

    // Test non-sequential elements
    rublas_inc = 2;
    res = asum_f64(5, x, rublas_inc);
    ASSERT_EQ(res, 25.0);
  }
  END_TEST();

  // asum_cf32
  START_TEST("asum_cf32");
  {
    cf32_t x[10] = {
        1.0f + 2.0f * I,  -3.0f - 4.0f * I, 5.0f + 6.0f * I,  -7.0f - 8.0f * I,
        9.0f + 10.0f * I, 1.0f - 2.0f * I,  -3.0f + 4.0f * I, 5.0f - 6.0f * I,
        -7.0f + 8.0f * I, 9.0f - 10.0f * I,
    };
    f32_t res;

    // Test sequential elements
    rublas_inc = 1;
    res = asum_cf32(10, x, rublas_inc);
    ASSERT_EQ(res, 110.0f);

    // Test non-sequential elements
    rublas_inc = 2;
    res = asum_cf32(5, x, rublas_inc);
    ASSERT_EQ(res, 55.0f);
  }
  END_TEST();

  // asum_cf64
  START_TEST("asum_cf64");
  {
    cf64_t x[10] = {
        1.0 + 2.0 * I,  -3.0 - 4.0 * I, 5.0 + 6.0 * I,  -7.0 - 8.0 * I,
        9.0 + 10.0 * I, 1.0 - 2.0 * I,  -3.0 + 4.0 * I, 5.0 - 6.0 * I,
        -7.0 + 8.0 * I, 9.0 - 10.0 * I,
    };
    f64_t res;

    // Test sequential elements
    rublas_inc = 1;
    res = asum_cf64(10, x, rublas_inc);
    ASSERT_EQ(res, 110.0);

    // Test non-sequential elements
    rublas_inc = 2;
    res = asum_cf64(5, x, rublas_inc);
    ASSERT_EQ(res, 55.0);
  }
  END_TEST();

  // ####################################################
  // cblas
  // ####################################################

  CBLAS_INDEX cblas_inc;

  // cblas_sasum
  START_TEST("cblas_sasum");
  {
    float x[10] = {1.0f,  -2.0f, 3.0f,  -4.0f, 5.0f,
                   -6.0f, 7.0f,  -8.0f, 9.0f,  -10.0f};
    float res;

    // Test sequential elements
    cblas_inc = 1;
    res = cblas_sasum(10, x, cblas_inc);
    ASSERT_EQ(res, 55.0f);

    // Test non-sequential elements
    cblas_inc = 2;
    res = cblas_sasum(5, x, cblas_inc);
    ASSERT_EQ(res, 25.0f);
  }
  END_TEST();

  // cblas_dasum
  START_TEST("cblas_dasum");
  {
    double x[10] = {1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0, 9.0, -10.0};
    double res;

    // Test sequential elements
    cblas_inc = 1;
    res = cblas_dasum(10, x, cblas_inc);
    ASSERT_EQ(res, 55.0);

    // Test non-sequential elements
    cblas_inc = 2;
    res = cblas_dasum(5, x, cblas_inc);
    ASSERT_EQ(res, 25.0);
  }
  END_TEST();

  // cblas_scasum
  START_TEST("cblas_scasum");
  {
    float complex x[10] = {1.0f + 2.0f * I,  -3.0f - 4.0f * I, 5.0f + 6.0f * I,
                           -7.0f - 8.0f * I, 9.0f + 10.0f * I, 1.0f - 2.0f * I,
                           -3.0f + 4.0f * I, 5.0f - 6.0f * I,  -7.0f + 8.0f * I,
                           9.0f - 10.0f * I};
    float res;

    // Test sequential elements
    cblas_inc = 1;
    res = cblas_scasum(10, x, cblas_inc);
    ASSERT_EQ(res, 110.0f);

    // Test non-sequential elements
    cblas_inc = 2;
    res = cblas_scasum(5, x, cblas_inc);
    ASSERT_EQ(res, 55.0f);
  }
  END_TEST();

  // cblas_dzasum
  START_TEST("cblas_dzasum");
  {
    double complex x[10] = {1.0 + 2.0 * I,  -3.0 - 4.0 * I, 5.0 + 6.0 * I,
                            -7.0 - 8.0 * I, 9.0 + 10.0 * I, 1.0 - 2.0 * I,
                            -3.0 + 4.0 * I, 5.0 - 6.0 * I,  -7.0 + 8.0 * I,
                            9.0 - 10.0 * I};
    double res;

    // Test sequential elements
    cblas_inc = 1;
    res = cblas_dzasum(10, x, cblas_inc);
    ASSERT_EQ(res, 110.0);

    // Test non-sequential elements
    cblas_inc = 2;
    res = cblas_dzasum(5, x, cblas_inc);
    ASSERT_EQ(res, 55.0);
  }
  END_TEST();

  return 0;
}
