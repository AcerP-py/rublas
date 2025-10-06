#include <stdio.h>
#include <stdlib.h>
#include "c_tests.h"
#include "rublas.h"

int main() {
    double x[10] = {1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0};
    double y[10] = {21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0};

    cblas_daxpy(10, 2.0f, x, 1, y, 1);
    assert_eq(y[2], 29.0f);

    double y2[10] = {21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0};
    cblas_daxpy(4, 2.0f, x, 1, y2, 1);
    assert_eq(y2[2], 29.0f);

    double y3[10] = {21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0};
    cblas_daxpy(0, 2.0f, x, 1, y3, 1);
    assert_eq(y3[2], 23.0f);

    double y4[10] = {21.0, 22.0, 23.0, 24.0, 25.0, 26.0, 27.0, 28.0, 29.0, 30.0};
    cblas_daxpy(5, 2.0f, x, 2, y4, 2);
    assert_eq(y4[4], 35.0f);

    return 0;
}
