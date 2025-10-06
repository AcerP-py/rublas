# RuBLAS
A pure Rust implementation of BLAS!

## Status
!!! THIS PROJECT USES RUST NIGHTLY !!!
Currently working on BLAS level 1.

## Usage

### Rust
``` rust
use rublas::*;

const N: usize = 1000;

fn main() {
    let x: Vec<SFLOAT> = vec![1.5; N];
    let y: Vec<SFLOAT> = vec![2.7; N];

    let result: SFLOAT = sdot(N, &x, 1, &y, 1);

    println!("Result: {}", result);
}
```

### C/C++
``` c
#include <stdio.h>
#include <stdlib.h>
#include "rublas.h"

#define N 1000

int main() {
    float *x = malloc(N * sizeof(float));
    float *y = malloc(N * sizeof(float));

    if (!x || !y) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }

    for (int i = 0; i < N; ++i) {
        x[i] = 1.5f;
        y[i] = 2.7f;
    }

    float result = rublas_sdot(N, x, 1, y, 1); // or cblas_sdot(N, x, 1, y, 1)

    printf("Result: %f\n", result);

    free(x);
    free(y);

    return 0;
}
```

### Fortran
``` fortran
program fortran_main
    use iso_c_binding
    use rublas
    implicit none

    integer(c_int), parameter :: N = 1000
    real(c_float), dimension(N) :: x, y
    real(c_float) :: result
    integer(c_int) :: inc_x, inc_y

    x = 1.5_c_float
    y = 2.7_c_float
    inc_x = 1
    inc_y = 1

    result = rublas_sdot(N, x, inc_x, y, inc_y) !! or sdot(N, x, inc_x, y, inc_y)

    print *, "Result:", result
end program fortran_main
```

## Contribution
Contributions are welcome! All contributions will be licensed under the terms
in [LICENCE.md](LICENSE.md).
