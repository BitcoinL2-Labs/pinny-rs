use pinny::tag;

#[tag(tag1)]
#[tag(tag2)]
#[test]
fn test_compilation_failure() {}

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let cargo_path = std::path::Path::new(&manifest_dir).join("Cargo.toml");

    println!("CARGO: {}", cargo_path.display());
}
