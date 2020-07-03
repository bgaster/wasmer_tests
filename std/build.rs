extern crate cc;

fn main() {
    //let mut project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let t: String = std::env::var("TARGET").unwrap();
    if !t.eq("wasm32-unknown-unknown") {
        cc::Build::new()
        .file("src/intrinsics.c")
        .opt_level(3)
        .compile("libstd.a");
    }
}
