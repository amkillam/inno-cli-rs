fn main() {
    println!("cargo:rustc-link-search=../ps/build");
    println!("cargo:rustc-link-lib=static=inno-cli");
}
