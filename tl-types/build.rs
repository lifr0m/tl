use std::path::PathBuf;
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let out_file = out_dir.join("schema.rs");

    let schema = fs::read_to_string("schema.tl")?;
    let schema = tl_parser::parse_schema(&schema)?;
    let code = tl_generator::generate(&schema);

    fs::write(out_file, code)?;

    Ok(())
}
