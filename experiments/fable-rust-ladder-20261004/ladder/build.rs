//! Compile the walt42x delta-expectimax engine (csrc/turbo.cpp: copied from the walt42x snapshot, then
//! extended here with the policy/leaf hooks and the leaf memo) into a static library with the system clang++;
//! no new crates, no downloads.
use std::process::Command;
fn main() {
    let out = std::env::var("OUT_DIR").unwrap();
    let obj = format!("{out}/turbo.o");
    let lib = format!("{out}/libturbo.a");
    // LADDER_CXXFLAGS (optional, whitespace-separated) appends flags for the C++ engine only, e.g. `-mcpu=native`
    // or the two PGO steps (`-fprofile-instr-generate`, then `-fprofile-instr-use=<profdata>`); see build_pgo.sh.
    // Nothing here may change floating-point semantics (no fast-math): outputs are byte-compared against the plain build.
    let extra: Vec<String> = std::env::var("LADDER_CXXFLAGS").unwrap_or_default().split_whitespace().map(str::to_string).collect();
    let mut cargs: Vec<String> = ["-O3", "-std=c++17", "-fPIC", "-Wall", "-Wextra", "-Werror"].iter().map(|s| s.to_string()).collect();
    cargs.extend(extra); cargs.extend(["-c", "csrc/turbo.cpp", "-o", &obj].iter().map(|s| s.to_string()));
    let st = Command::new("clang++").args(&cargs).status().expect("clang++");
    assert!(st.success(), "turbo.cpp failed to compile");
    let st = Command::new("ar").args(["rcs", &lib, &obj]).status().expect("ar");
    assert!(st.success());
    println!("cargo:rustc-link-search=native={out}");
    println!("cargo:rustc-link-lib=static=turbo");
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rerun-if-changed=csrc/turbo.cpp");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=LADDER_CXXFLAGS");
    if std::env::var("LADDER_CXXFLAGS").unwrap_or_default().contains("-fprofile-instr-generate") { println!("cargo:rustc-link-arg=-fprofile-instr-generate"); }
    if let Some(f) = std::env::var("LADDER_CXXFLAGS").ok().and_then(|v| v.split_whitespace().find_map(|a| a.strip_prefix("-fprofile-instr-use=").map(str::to_string))) { println!("cargo:rerun-if-changed={f}"); }
}
