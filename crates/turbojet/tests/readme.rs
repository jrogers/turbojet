//! The readme's first program is `examples/quickstart.rs`, which the build compiles: this keeps
//! the two the same.

#[test]
fn the_readme_shows_the_quickstart_example() {
    let readme = include_str!("../../../README.md");
    let example = include_str!("../examples/quickstart.rs");
    let program = &example[example.find("use std::sync::Arc;").expect("the example's first import")..];
    assert!(readme.contains(program), "README.md's quick start differs from examples/quickstart.rs");
}
