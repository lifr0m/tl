mod output;
mod generate;

use generate::generate_schema;
use output::Output;

pub fn generate(schema: &tl_parser::Schema) -> String {
    let mut output = Output::new(4, 0);
    generate_schema(&mut output, schema);
    output.destruct()
}
