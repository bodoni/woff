fn main() {
    woff2();
}

fn woff2() {
    let build = cmake::Config::new("woff2/source")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();
    println!(
        "cargo:rustc-link-search=native={}",
        build.join("lib").display(),
    );
    println!("cargo:rustc-link-lib=static=woff2common");
    println!("cargo:rustc-link-lib=static=woff2enc");
    println!("cargo:rustc-link-lib=static=woff2dec");
    cc::Build::new()
        .file("woff2/wrapper/woff2.cpp")
        .include("woff2/wrapper")
        .include(build.join("include"))
        .cpp(true)
        .compile("woff2");
}
