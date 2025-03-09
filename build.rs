extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    cc::Build::new()
        .std("c17") // https://docs.rs/cc/1.2.16/cc/struct.Build.html#method.std
        .define("WINDOWS_OS", None) // To avoid `ext/scrypt/crypto_scrypt.c(33): fatal error C1083: Cannot open include file: 'sys/mman.h': No such file or directory`
        .include("ext/scrypt")
        .file("ext/scrypt/crypto_scrypt.c")
        .file("ext/scrypt/crypto_scrypt_smix.c")
        .file("ext/scrypt/sha256.c")
        .file("ext/scrypt/insecure_memzero.c")
        .file("ext/scrypt/warnp.c")
        .compile("libscrypt.a");

    println!(
        "cargo:rustc-link-search=native={}",
        out_path.to_str().unwrap()
    );
    println!("cargo:rustc-link-lib=static=scrypt");
}
