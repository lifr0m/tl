use anyhow::Context;
use std::path::PathBuf;
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let name_list = ["api"];
    let out_dir = out_dir.join("schemas");

    if !out_dir.exists() {
        fs::create_dir(&out_dir)?;
    }

    for name in name_list {
        let out_file = out_dir.join(format!("{name}.rs"));

        let schema = fs::read_to_string(format!("schemas/{name}.tl"))
            .with_context(|| format!("failed to read schema: {name}"))?;
        let schema = tl_parser::parse_schema(&schema)
            .with_context(|| format!("failed to parse schema: {name}"))?;
        let code = tl_generator::generate(&schema);

        fs::write(out_file, code)
            .with_context(|| format!("failed to write generated code for schema: {name}"))?;
    }

    Ok(())
}
