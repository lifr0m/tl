use anyhow::Context;
use std::path::PathBuf;
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let out_file = out_dir.join("schema.rs");

    let ci = env::var("CI")
        .unwrap_or_else(|_| String::from("false"))
        == "true";

    let schema = if ci {
        String::from("")
    } else {
        fs::read_to_string("schema.tl")
            .context("Failed to read schema.tl")?
    };
    let schema = tl_parser::parse_schema(&schema)
        .context("Failed to parse schema")?;
    let code = tl_generator::generate(&schema);

    fs::write(out_file, code)
        .context("Failed to write generated code")?;

    Ok(())
}
