use std::{process::Command, sync::Once};

#[cfg(target_os = "linux")]
use headcrab::{
    symbol::RelocatedDwarf,
    target::{LinuxTarget, UnixTarget},
};

static TESTEES_BUILD: Once = Once::new();

/// Ensure that all testees are built.
pub fn ensure_testees() {
    TESTEES_BUILD.call_once(|| {
        let status = std::process::Command::new("make")
            .current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/testees"))
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        assert!(status.success());
    });
}

#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub fn launch(path: &str) -> LinuxTarget {
    let (target, status) = LinuxTarget::launch(Command::new(path)).unwrap();
    match status {
        nix::sys::wait::WaitStatus::Stopped(_, nix::sys::signal::SIGTRAP) => {}
        _ => panic!("Status: {:?}", status),
    }
    target
}

#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub fn continue_to_end(target: &LinuxTarget) {
    match target.unpause().expect("Failed to unpause target") {
        nix::sys::wait::WaitStatus::Exited(_, 0) => {}
        status => panic!("Unexpected signal: Status: {:?}", status),
    }
}

/// Turn the `pause` instruction inside the `breakpoint` function into a breakpoint.
#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub fn patch_breakpoint(target: &LinuxTarget, debuginfo: &RelocatedDwarf) {
    // Get the address of the `breakpoint` function.
    let breakpoint_addr = debuginfo.get_symbol_address("breakpoint").unwrap() + 4 /* prologue */;
    target
        .set_breakpoint(breakpoint_addr)
        .expect("Failed to patch breakpoint");
}

#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub fn current_ip(target: &LinuxTarget) -> u64 {
    target.read_regs().expect("could not read registers").ip()
}

#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub fn ws_sigtrap(target: &LinuxTarget) -> nix::sys::wait::WaitStatus {
    nix::sys::wait::WaitStatus::Stopped(target.pid(), nix::sys::signal::SIGTRAP)
}
