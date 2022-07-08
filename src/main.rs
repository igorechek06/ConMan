use std::process::exit;

#[cfg(target_os = "linux")]
fn main() {
    exit(conman::run());
}
