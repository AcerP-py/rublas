fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const GIT_HASH: &str = env!("GIT_HASH");
    const BUILD_TIMESTAMP: &str = env!("BUILD_TIMESTAMP");
    const ARCH: &str = env!("ARCH");
    const MICRO_ARCH: &str = env!("MICRO_ARCH");

    println!("Rublas v{}+{} {}", VERSION, GIT_HASH, BUILD_TIMESTAMP);
    println!("Target: {}/{}", ARCH, MICRO_ARCH);
}
