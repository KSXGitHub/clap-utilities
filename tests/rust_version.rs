use cargo_toml::Manifest;
use pipe_trait::Pipe;

#[test]
fn sync_rust_version() {
    let toolchain = include_str!("../rust-toolchain").trim_end();

    let msrv = include_str!("../Cargo.toml")
        .pipe(Manifest::from_str)
        .expect("parse Cargo.toml")
        .package
        .expect("read .package")
        .rust_version
        .expect("read .package.rust-version");
    let msrv = msrv
        .get()
        .expect("extract .package.rust-version")
        .trim_end();

    assert_eq!(toolchain, msrv);
}
