use std::env;

fn main() {
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    if target.contains("x86_64") {
        println!("cargo:rustc-link-lib=dylib=mantle64");
        println!("cargo:rustc-link-search=native=C:\\Windows\\System32");

    } else if target.contains("i686") {
        println!("cargo:rustc-link-lib=dylib=mantle32");

        if host.contains("x86_64") {
            println!("cargo:rustc-link-search=native=C:\\Windows\\SysWOW64");
        } else {
            println!("cargo:rustc-link-search=native=C:\\Windows\\System32");
        }

    } else {
        unreachable!();
    }
}
