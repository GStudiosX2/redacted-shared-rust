use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=linker.ld");
    println!("cargo::rerun-if-changed=vendor/RedactedOS");

    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy(format!("{}/linker.ld", env::var("CARGO_MANIFEST_DIR").unwrap()), out.join("link.x")).unwrap();
    println!("cargo::rustc-link-search={}", out.display());

    // TODO: this is probably not the proper way to do it
    unsafe {
        env::set_var("CC", "aarch64-none-elf-gcc");
        env::set_var("CXX", "aarch64-none-elf-g++");
        env::set_var("AR", "aarch64-none-elf-ar");
    }

    let mut shared_compiler = cc::Build::new();
    shared_compiler.files(glob::glob("vendor/RedactedOS/shared/**/*.[cS]").expect("failed to read vendor/RedactedOS").filter_map(Result::ok));
    // -ffreestanding -nostdlib -fno-exceptions -fno-unwind-tables \
    //  -fno-asynchronous-unwind-tables -g -O0 -Wall -Wextra \
    //  -Wno-unused-parameter -Wno-address-of-packed-member \
    //  -mcpu=cortex-a72 -Werror -Wno-error=sized-deallocation
    shared_compiler.flags([
        "-ffreestanding", "-nostdlib", "-fno-exceptions", "-fno-unwind-tables",
        "-fno-asynchronous-unwind-tables", "-Wno-unused-parameter", "-Wno-address-of-packed-member", 
        "-mcpu=cortex-a72", "-Werror", "-Wno-error=sized-deallocation"
    ]);
    shared_compiler.include("vendor/RedactedOS/shared");
    shared_compiler.include("vendor/RedactedOS/kernel");
    shared_compiler.std("c99");
    shared_compiler.compile("shared");
}