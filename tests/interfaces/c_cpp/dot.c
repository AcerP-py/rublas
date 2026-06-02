#include "assert.h"
#include "cblas.h"
#include "rublas.h"

int main() {

  // ####################################################
  // rublas
  // ####################################################

  usize_t rublas_inc;

  // dot_f32
  START_TEST("dot_f32");
  {
    f32_t x[10];
    f32_t y[10];
    f32_t res;

    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);

    // Test sequential elements
    rublas_inc = 1;
    res = dot_f32(10, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 20.0f);

    // Test non-sequential elements
    rublas_inc = 2;
    res = dot_f32(5, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 10.0f);
  }
  END_TEST();

  // dot_f32_as_f64
  START_TEST("dot_f32_as_f64");
  {
    f32_t x[10];
    f32_t y[10];
    f64_t res;

    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);

    // Test sequential elements
    rublas_inc = 1;
    res = dot_f32_as_f64(10, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 20.0);

    // Test non-sequential elements
    rublas_inc = 2;
    res = dot_f32_as_f64(5, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 10.0);
  }
  END_TEST();

  // dot_f64
  START_TEST("dot_f64");
  {
    f64_t x[10];
    f64_t y[10];
    f64_t res;

    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY(y, 10, 2.0);

    // Test sequential elements
    rublas_inc = 1;
    res = dot_f64(10, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 20.0);

    // Test non-sequential elements
    rublas_inc = 2;
    res = dot_f64(5, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 10.0);
  }
  END_TEST();

  // dot_cf32
  START_TEST("dot_cf32");
  {
    cf32_t x[10];
    cf32_t y[10];
    cf32_t res;

    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);
    INIT_ARRAY(y, 10, 1.0f + 1.0f * I);

    // Test sequential elements
    rublas_inc = 1;
    res = dot_cf32(10, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 0.0f + 20.0f * I);

    // Test non-sequential elements
    rublas_inc = 2;
    res = dot_cf32(5, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 0.0f + 10.0f * I);
  }
  END_TEST();

  // dot_cf32_conj
  START_TEST("dot_cf32_conj");
  {
    cf32_t x[10];
    cf32_t y[10];
    cf32_t res;

    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);
    INIT_ARRAY(y, 10, 1.0f + 1.0f * I);

    // Test sequential elements
    rublas_inc = 1;
    res = dot_cf32_conj(10, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 20.0f + 0.0f * I);

    // Test non-sequential elements
    rublas_inc = 2;
    res = dot_cf32_conj(5, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 10.0f + 0.0f * I);
  }
  END_TEST();

  // dot_cf64
  START_TEST("dot_cf64");
  {
    cf64_t x[10];
    cf64_t y[10];
    cf64_t res;

    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);
    INIT_ARRAY(y, 10, 1.0 + 1.0 * I);

    // Test sequential elements
    rublas_inc = 1;
    res = dot_cf64(10, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 0.0 + 20.0 * I);

    // Test non-sequential elements
    rublas_inc = 2;
    res = dot_cf64(5, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 0.0 + 10.0 * I);
  }
  END_TEST();

  // dot_cf64_conj
  START_TEST("dot_cf64_conj");
  {
    cf64_t x[10];
    cf64_t y[10];
    cf64_t res;

    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);
    INIT_ARRAY(y, 10, 1.0 + 1.0 * I);

    // Test sequential elements
    rublas_inc = 1;
    res = dot_cf64_conj(10, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 20.0 + 0.0 * I);

    // Test non-sequential elements
    rublas_inc = 2;
    res = dot_cf64_conj(5, x, rublas_inc, y, rublas_inc);
    ASSERT_EQ(res, 10.0 + 0.0 * I);
  }
  END_TEST();

  // ####################################################
  // cblas
  // ####################################################

  CBLAS_INDEX cblas_inc;

  // cblas_sdot
  START_TEST("cblas_sdot");
  {
    float x[10];
    float y[10];
    float res;

    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);

    // Test sequential elements
    cblas_inc = 1;
    res = cblas_sdot(10, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ(res, 20.0f);

    // Test non-sequential elements
    cblas_inc = 2;
    res = cblas_sdot(5, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ(res, 10.0f);
  }
  END_TEST();

  // cblas_sdsdot
  START_TEST("cblas_sdsdot");
  {
    float x[10];
    float y[10];
    float alpha = 1.0f;
    float res;

    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);

    // Test sequential elements
    cblas_inc = 1;
    res = cblas_sdsdot(10, alpha, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ(res, 21.0f);

    // Test non-sequential elements
    cblas_inc = 2;
    res = cblas_sdsdot(5, alpha, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ(res, 11.0f);
  }
  END_TEST();

  // cblas_dsdot
  START_TEST("cblas_dsdot");
  {
    float x[10];
    float y[10];
    double res;

    INIT_ARRAY(x, 10, 1.0f);
    INIT_ARRAY(y, 10, 2.0f);

    // Test sequential elements
    cblas_inc = 1;
    res = cblas_dsdot(10, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ(res, 20.0);

    // Test non-sequential elements
    cblas_inc = 2;
    res = cblas_dsdot(5, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ(res, 10.0);
  }
  END_TEST();

  // cblas_ddot
  START_TEST("cblas_ddot");
  {
    double x[10];
    double y[10];
    double res;

    INIT_ARRAY(x, 10, 1.0);
    INIT_ARRAY(y, 10, 2.0);

    // Test sequential elements
    cblas_inc = 1;
    res = cblas_ddot(10, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ(res, 20.0);

    // Test non-sequential elements
    cblas_inc = 2;
    res = cblas_ddot(5, x, cblas_inc, y, cblas_inc);
    ASSERT_EQ(res, 10.0);
  }
  END_TEST();

  // cblas_cdotu_sub
  START_TEST("cblas_cdotu_sub");
  {
    float complex x[10];
    float complex y[10];
    float complex res;

    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);
    INIT_ARRAY(y, 10, 1.0f + 1.0f * I);

    // Test sequential elements
    cblas_inc = 1;
    cblas_cdotu_sub(10, x, cblas_inc, y, cblas_inc, &res);
    ASSERT_EQ(res, 0.0f + 20.0f * I);

    // Test non-sequential elements
    cblas_inc = 2;
    cblas_cdotu_sub(5, x, cblas_inc, y, cblas_inc, &res);
    ASSERT_EQ(res, 0.0f + 10.0f * I);
  }
  END_TEST();

  // cblas_cdotc_sub
  START_TEST("cblas_cdotc_sub");
  {
    float complex x[10];
    float complex y[10];
    float complex res;

    INIT_ARRAY(x, 10, 1.0f + 1.0f * I);
    INIT_ARRAY(y, 10, 1.0f + 1.0f * I);

    // Test sequential elements
    cblas_inc = 1;
    cblas_cdotc_sub(10, x, cblas_inc, y, cblas_inc, &res);
    ASSERT_EQ(res, 20.0f + 0.0f * I);

    // Test non-sequential elements
    cblas_inc = 2;
    cblas_cdotc_sub(5, x, cblas_inc, y, cblas_inc, &res);
    ASSERT_EQ(res, 10.0f + 0.0f * I);
  }
  END_TEST();

  // cblas_zdotu_sub
  START_TEST("cblas_zdotu_sub");
  {
    double complex x[10];
    double complex y[10];
    double complex res;

    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);
    INIT_ARRAY(y, 10, 1.0 + 1.0 * I);

    // Test sequential elements
    cblas_inc = 1;
    cblas_zdotu_sub(10, x, cblas_inc, y, cblas_inc, &res);
    ASSERT_EQ(res, 0.0 + 20.0 * I);

    // Test non-sequential elements
    cblas_inc = 2;
    cblas_zdotu_sub(5, x, cblas_inc, y, cblas_inc, &res);
    ASSERT_EQ(res, 0.0 + 10.0 * I);
  }
  END_TEST();

  // cblas_zdotc_sub
  START_TEST("cblas_zdotc_sub");
  {
    double complex x[10];
    double complex y[10];
    double complex res;

    INIT_ARRAY(x, 10, 1.0 + 1.0 * I);
    INIT_ARRAY(y, 10, 1.0 + 1.0 * I);

    // Test sequential elements
    cblas_inc = 1;
    cblas_zdotc_sub(10, x, cblas_inc, y, cblas_inc, &res);
    ASSERT_EQ(res, 20.0 + 0.0 * I);

    // Test non-sequential elements
    cblas_inc = 2;
    cblas_zdotc_sub(5, x, cblas_inc, y, cblas_inc, &res);
    ASSERT_EQ(res, 10.0 + 0.0 * I);
  }
  END_TEST();

  return 0;
}
