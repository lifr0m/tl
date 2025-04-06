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
        String::from("type Message id:int32 text:string? photos:[bytes] sent_at:time = Message
type User id:int64 verified:bool rating:float = User
type UserEmpty id:int64 = User

error InvalidUserId user_id:int64
error TooLongText text:string max_length:int32

func get_users user_ids:[int64] = [User]
func send_message user_id:int64 text:string? photos:[bytes] = Message")
    } else {
        fs::read_to_string("schema.tl")
            .context("failed to read schema.tl")?
    };
    let schema = tl_parser::parse_schema(&schema)
        .context("failed to parse schema")?;
    let code = tl_generator::generate(&schema);

    fs::write(out_file, code)
        .context("failed to write generated code")?;

    Ok(())
}
