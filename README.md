# RuBLAS
A pure Rust implementation of BLAS!
(With C/C++ interfaces).

## Status
!!! THIS PROJECT USES RUST NIGHTLY !!!
This project still needs a LOT of development.
I am currently working on BLAS level 1.

## Install
You will need a working install of rust with cargo.
``` bash
git clone https://github.com/AcerP-py/rublas.git
cd rublas
make install PREFIX=<install_path>
```

## Usage

### Rust
``` rust
use rublas::*;

const N: usize = 1000;

fn main() {
    let x: Vec<f32> = vec![1.5; N];
    let y: Vec<f32> = vec![2.7; N];

    let result: f32 = dot(N, &x, 1, &y, 1);

    println!("Result: {}", result);
}
```

### C/C++
``` c
#include <stdio.h>
#include <stdlib.h>
#include "rublas.h"
// cblas.h is also available

#define N 1000

int main() {
    f32_t *x = malloc(N * sizeof(f32_t));
    f32_t *y = malloc(N * sizeof(f32_t));

    if (!x || !y) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }

    for (int i = 0; i < N; ++i) {
        x[i] = 1.5f;
        y[i] = 2.7f;
    }

    f32_t result = dot_f32(N, x, 1, y, 1);

    printf("Result: %f\n", result);

    free(x);
    free(y);

    return 0;
}
```

## Contribution
Contributions are welcome! All contributions will be licensed under the terms
in [LICENCE.md](LICENSE.md).
