# RuBLAS
A pure Rust implementation of BLAS!

## Status
!!! THIS PROJECT USES RUST NIGHTLY !!!
Currently working on BLAS level 1.

## Usage
``` rust
use rublas::*;

const N: usize = 1000;

fn main() {
    let x: Vec<f32> = vec![1.5; N];
    let y: Vec<f32> = vec![2.7; N];

    let result: f32 = sdot(N, &x, 1, &y, 1);

    println!("Result: {}", result);
}
```
``` bash
$ cargo run
...
Result: 4049.9956
```

## Contribution
Contributions are welcome! All contributions will be licensed under the terms
in [LICENCE.md](LICENSE.md).
