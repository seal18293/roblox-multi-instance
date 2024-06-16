fn main() {
    println!("cargo::rerun-if-changed=src-cpp/mutex.cpp");
    cc::Build::new().file("src-cpp/mutex.cpp").compile("cpp");
}
