fn main() {
    println!("cargo:rustc-link-search=./");
    println!("cargo:rustc-link-lib=static=xlog");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-flags=-l c++");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreWLAN");
    println!("cargo:rustc-link-lib=framework=SystemConfiguration");
}