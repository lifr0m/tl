mod output;
mod generate;
mod hash;

use generate::Generate;
use output::Output;

pub fn generate(schema: &tl_parser::Schema) -> String {
    let mut output = Output::new(4, 0);
    schema.generate(&mut output);
    output.destruct()
}
