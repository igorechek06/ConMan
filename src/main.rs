use std::process::exit;

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
fn main() {
    exit(conman::run());
}
