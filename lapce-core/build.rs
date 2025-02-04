use std::{env, fs, path::Path};

use anyhow::Result;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=RELEASE_TAG_NAME");
    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");

    #[cfg(not(debug_assertions))]
    let (tag, release, version) = {
        let tag = env::var("RELEASE_TAG_NAME").unwrap_or(String::from("nightly"));

        let (release, version) = if tag.starts_with('v') {
            ("Stable", env::var("CARGO_PKG_VERSION")?)
        } else {
            ("Nightly", tag.clone())
        };

        (tag, release, version)
    };

    #[cfg(debug_assertions)]
    let (tag, release, version) = (String::new(), "Debug", String::from("debug"));

    let meta_file = Path::new(&env::var("OUT_DIR")?).join("meta.rs");

    #[rustfmt::skip]
    let meta = format!(r#"
        pub const TAG: &str = "{tag}";
        pub const NAME: &str = "Lapce-{release}";
        pub const VERSION: &str = "{version}";
        pub const RELEASE: ReleaseType = ReleaseType::{release};
    "#);

    fs::write(meta_file, meta)?;

    Ok(())
}
