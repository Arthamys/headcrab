//! This is a simple test to running a syscall in a child process.

mod test_utils;

#[cfg(target_os = "linux")]
use headcrab::{target::LinuxTarget, target::UnixTarget};

static BIN_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/testees/hello");

// FIXME: Running this test just for linux because of privileges issue on macOS. Enable for everything after fixing.
#[cfg(target_os = "linux")]
#[test]
fn syscall() -> Result<(), Box<dyn std::error::Error>> {
    test_utils::ensure_testees();

    let target = LinuxTarget::launch(BIN_PATH)?;

    println!(
        "{}\n",
        std::fs::read_to_string(format!("/proc/{}/maps", target.pid()))?
    );

    let addr = target
        .mmap(
            0 as *mut _,
            1 << 20,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
            0,
            0,
        )
        .unwrap();

    for line in std::fs::read_to_string(format!("/proc/{}/maps", target.pid()))?.lines() {
        if line.starts_with(&format!("{:08x}-", addr)) {
            // Found mapped addr
            target.unpause()?;
            return Ok(());
        }
    }

    panic!(
        "\n{}\n",
        std::fs::read_to_string(format!("/proc/{}/maps", target.pid()))?
    );
}
