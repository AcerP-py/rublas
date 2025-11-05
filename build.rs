fn main() {
    // generate build time configuration
    let git_hash: Vec<u8> = std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .expect("Failed to retrieve git hash")
        .stdout;
    let git_hash: String = String::from_utf8(git_hash)
        .expect("Could not parse UTF-8 git version")
        .trim()
        .to_string();

    let timestamp: Vec<u8> = std::process::Command::new("date")
        .args(&["-u", "+%m/%d/%y %H:%M:%S UTC %Y"])
        .output()
        .expect("Failed to retrieve date")
        .stdout;
    let timestamp: String = String::from_utf8(timestamp)
        .expect("Could not parse UTF-8 date")
        .trim()
        .to_string();

    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=BUILD_TIMESTAMP={}", timestamp);
    println!("cargo:rustc-env=ARCH={}", std::env::consts::ARCH);
    println!("cargo:rustc-env=MICRO_ARCH={}", "generic"); // not yet implemented
}
