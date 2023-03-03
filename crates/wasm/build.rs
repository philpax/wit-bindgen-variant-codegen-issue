use std::path::Path;

/// Copies all files in the wit/ folder to all guests, and generates wit ahead of time
/// for Rust guests.
fn main() {
    println!("cargo:rerun-if-changed=wit");

    use wit_bindgen_core::{wit_parser::Resolve, Files};

    let guest_path = Path::new("../../guest");

    let mut generator = wit_bindgen_rust::Opts::default().build();
    let mut resolve = Resolve::new();
    let pkg = resolve.push_dir(Path::new("wit")).unwrap().0;

    let mut files = Files::default();
    let world = resolve.select_world(pkg, Some("main.server")).unwrap();
    generator.generate(&resolve, world, &mut files);

    for (filename, contents) in files.iter() {
        let contents = std::str::from_utf8(contents).unwrap();

        // temp ugly hack: inject our custom definitions of wit-bindgen helpers in so that we don't have
        // a Git dependency
        let contents = contents
            .lines()
            .map(|s| {
                if s.trim().starts_with("pub mod") {
                    format!("{s} use super::wit_bindgen;")
                } else {
                    s.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        std::fs::write(guest_path.join("src").join(filename), contents).unwrap();
    }
}
