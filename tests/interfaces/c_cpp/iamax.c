#include "assert.h"
#include "cblas.h"
#include "rublas.h"

int main() {

  // ####################################################
  // rublas
  // ####################################################

  usize_t rublas_inc;
  usize_t rublas_res;

  // iamax_f32
  START_TEST("iamax_f32");
  {
    f32_t x[10] = {1.0f, 9.0f,  3.0f, 4.0f, 3.0f,
                   6.0f, 11.0f, 8.0f, 9.0f, 10.0f};

    // Test sequential elements
    rublas_inc = 1;
    rublas_res = iamax_f32(10, x, rublas_inc);
    ASSERT_EQ(rublas_res, 6);

    // Test non-sequential elements
    rublas_inc = 2;
    rublas_res = iamax_f32(5, x, rublas_inc);
    ASSERT_EQ(rublas_res, 3);
  }
  END_TEST();

  // iamax_f64
  START_TEST("iamax_f64");
  {
    f64_t x[10] = {1.0, 9.0, 3.0, 4.0, 3.0, 6.0, 11.0, 8.0, 9.0, 10.0};

    // Test sequential elements
    rublas_inc = 1;
    rublas_res = iamax_f64(10, x, rublas_inc);
    ASSERT_EQ(rublas_res, 6);

    // Test non-sequential elements
    rublas_inc = 2;
    rublas_res = iamax_f64(5, x, rublas_inc);
    ASSERT_EQ(rublas_res, 3);
  }
  END_TEST();

  // iamax_cf32
  START_TEST("iamax_cf32");
  {
    cf32_t x[10] = {
        1.0f + 1.0f * I, 1.0f + 9.0f * I,  3.0f + 1.0f * I, 4.0f + 1.0f * I,
        3.0f + 1.0f * I, 6.0f + 1.0f * I,  2.0f + 1.0f * I, 8.0f + 3.0f * I,
        9.0f + 1.0f * I, 10.0f + 1.0f * I,
    };

    // Test sequential elements
    rublas_inc = 1;
    rublas_res = iamax_cf32(10, x, rublas_inc);
    ASSERT_EQ(rublas_res, 7);

    // Test non-sequential elements
    rublas_inc = 2;
    rublas_res = iamax_cf32(5, x, rublas_inc);
    ASSERT_EQ(rublas_res, 4);
  }
  END_TEST();

  // iamax_cf64
  START_TEST("iamax_cf64");
  {
    cf64_t x[10] = {
        1.0 + 1.0 * I, 1.0 + 9.0 * I,  3.0 + 1.0 * I, 4.0 + 1.0 * I,
        3.0 + 1.0 * I, 6.0 + 1.0 * I,  2.0 + 1.0 * I, 8.0 + 3.0 * I,
        9.0 + 1.0 * I, 10.0 + 1.0 * I,
    };

    // Test sequential elements
    rublas_inc = 1;
    rublas_res = iamax_cf64(10, x, rublas_inc);
    ASSERT_EQ(rublas_res, 7);

    // Test non-sequential elements
    rublas_inc = 2;
    rublas_res = iamax_cf64(5, x, rublas_inc);
    ASSERT_EQ(rublas_res, 4);
  }
  END_TEST();

  // ####################################################
  // cblas
  // ####################################################

  CBLAS_INDEX cblas_inc;
  CBLAS_INDEX cblas_res;

  // cblas_isamax
  START_TEST("cblas_isamax");
  {
    float x[10] = {1.0f, 9.0f,  3.0f, 4.0f, 3.0f,
                   6.0f, 11.0f, 8.0f, 9.0f, 10.0f};

    // Test sequential elements
    cblas_inc = 1;
    cblas_res = cblas_isamax(10, x, cblas_inc);
    ASSERT_EQ(cblas_res, 6);

    // Test non-sequential elements
    cblas_inc = 2;
    cblas_res = cblas_isamax(5, x, cblas_inc);
    ASSERT_EQ(cblas_res, 3);
  }
  END_TEST();

  // cblas_idamax
  START_TEST("cblas_idamax");
  {
    double x[10] = {1.0, 9.0, 3.0, 4.0, 3.0, 6.0, 11.0, 8.0, 9.0, 10.0};

    // Test sequential elements
    cblas_inc = 1;
    cblas_res = cblas_idamax(10, x, cblas_inc);
    ASSERT_EQ(cblas_res, 6);

    // Test non-sequential elements
    cblas_inc = 2;
    cblas_res = cblas_idamax(5, x, cblas_inc);
    ASSERT_EQ(cblas_res, 3);
  }
  END_TEST();

  // cblas_icamax
  START_TEST("cblas_icamax");
  {
    float complex x[10] = {
        1.0f + 1.0f * I, 1.0f + 9.0f * I,  3.0f + 1.0f * I, 4.0f + 1.0f * I,
        3.0f + 1.0f * I, 6.0f + 1.0f * I,  2.0f + 1.0f * I, 8.0f + 3.0f * I,
        9.0f + 1.0f * I, 10.0f + 1.0f * I,
    };

    // Test sequential elements
    cblas_inc = 1;
    cblas_res = cblas_icamax(10, x, cblas_inc);
    ASSERT_EQ(cblas_res, 7);

    // Test non-sequential elements
    cblas_inc = 2;
    cblas_res = cblas_icamax(5, x, cblas_inc);
    ASSERT_EQ(cblas_res, 4);
  }
  END_TEST();

  // cblas_izamax
  START_TEST("cblas_izamax");
  {
    double complex x[10] = {
        1.0 + 1.0 * I, 1.0 + 9.0 * I,  3.0 + 1.0 * I, 4.0 + 1.0 * I,
        3.0 + 1.0 * I, 6.0 + 1.0 * I,  2.0 + 1.0 * I, 8.0 + 3.0 * I,
        9.0 + 1.0 * I, 10.0 + 1.0 * I,
    };

    // Test sequential elements
    cblas_inc = 1;
    cblas_res = cblas_izamax(10, x, cblas_inc);
    ASSERT_EQ(cblas_res, 7);

    // Test non-sequential elements
    cblas_inc = 2;
    cblas_res = cblas_izamax(5, x, cblas_inc);
    ASSERT_EQ(cblas_res, 4);
  }
  END_TEST();

  return 0;
}
