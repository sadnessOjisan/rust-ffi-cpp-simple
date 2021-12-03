use std::env;
fn main(){
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    eprintln!("{}",project_dir);
    println!("cargo:rustc-link-search={}/target/", project_dir); // the "-L" flag
    println!("cargo:rustc-link-lib=test"); // the "-l" flag
}
