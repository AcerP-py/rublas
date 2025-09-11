use cfg_if;

/*
 * modules for specific architectures
 */

// generic
// generic module should always be available to fill in gaps
// so we allow unused code in the generic module
#[allow(dead_code)]
mod generic;

// x86_64
cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
    }
}

/*
 * kernel definitions
 */

// sdot
cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub fn k_sdot(n: usize, x: &[f32], inc_x: usize, y: &[f32], inc_y: usize) -> f32 { x86_64::ak_sdot(n, x, inc_x, y, inc_y) }
    } else {
        pub fn k_sdot(n: usize, x: &[f32], inc_x: usize, y: &[f32], inc_y: usize) -> f32 { generic::ak_sdot(n, x, inc_x, y, inc_y) }
    }
}
