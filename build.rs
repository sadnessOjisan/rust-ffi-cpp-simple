fn main() {
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++20")
        .warnings(true)
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-v")
        .flag("-g")
        .file("./src/test.cpp")
        .compile("libtest.a");
}
