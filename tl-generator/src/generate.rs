use crate::output::Output;
use convert_case::{Case, Casing};
use tl_parser::*;

pub(crate) trait Generate {
    fn generate(&self, output: &mut Output);
}

impl Generate for Schema {
    fn generate(&self, output: &mut Output) {
        output.write_line(|o| o.write("pub mod types {"));
        output.with_indent(|o| {
            for def in &self.types {
                def.generate(o);
                o.write("\n");
            }
        });
        output.write_line(|o| o.write("}"));

        output.write("\n");

        output.write_line(|o| o.write("pub mod errors {"));
        output.with_indent(|o| {
            for def in &self.errors {
                def.generate(o);
                o.write("\n");
            }
        });
        output.write_line(|o| o.write("}"));

        output.write("\n");

        output.write_line(|o| o.write("pub mod functions {"));
        output.with_indent(|o| {
            for def in &self.functions {
                def.generate(o);
                o.write("\n");
            }
        });
        output.write_line(|o| o.write("}"));
    }
}

impl Generate for TypeDefinition {
    fn generate(&self, output: &mut Output) {
        generate_definition(output, self.id, self.name.clone(), &self.fields, None);
    }
}

impl Generate for ErrorDefinition {
    fn generate(&self, output: &mut Output) {
        generate_definition(output, self.id, self.name.clone(), &self.fields, None);
    }
}

impl Generate for FunctionDefinition {
    fn generate(&self, output: &mut Output) {
        generate_definition(output, self.id, self.name.clone(), &self.args, Some(&self.typ));
    }
}

impl Generate for Type {
    fn generate(&self, output: &mut Output) {
        match self {
            Self::Int32 => output.write("i32"),
            Self::Int64 => output.write("i64"),
            Self::Float => output.write("f64"),
            Self::Bool => output.write("bool"),
            Self::String => output.write("String"),
            Self::Bytes => output.write("Vec::<u8>"),
            Self::Time => output.write("std::time::SystemTime"),
            Self::Vector(inner) => {
                output.write("Vec::<");
                inner.generate(output);
                output.write(">");
            }
            Self::Option(inner) => {
                output.write("Option::<");
                inner.generate(output);
                output.write(">");
            }
            Self::Defined(defined) => {
                output.write("crate::types::");
                output.write(defined);
            }
        };
    }
}

fn generate_definition(
    output: &mut Output,
    id: u32,
    mut name: String,
    fields: &Vec<Field>,
    typ: Option<&Type>,
) {
    if typ.is_some() {
        name = name.to_case(Case::Pascal);
    }

    output.write_line(|o| {
        o.write("pub struct ");
        o.write(&name);
        o.write(" {");
    });
    output.with_indent(|o| {
        for f in fields {
            o.write_line(|o| {
                o.write("pub ");
                o.write(&f.name);
                o.write(": ");
                f.typ.generate(o);
                o.write(",");
            });
        }
    });
    output.write_line(|o| o.write("}"));

    output.write("\n");

    output.write_line(|o| {
        o.write("impl crate::Identify for ");
        o.write(&name);
        o.write(" {");
    });
    output.with_indent(|o| {
        o.write_line(|o| {
            o.write("const ID: u32 = ");
            o.write(&id.to_string());
            o.write(";");
        });
    });
    output.write_line(|o| o.write("}"));

    output.write("\n");

    output.write_line(|o| {
        o.write("impl crate::Serialize for ");
        o.write(&name);
        o.write(" {");
    });
    output.with_indent(|o| {
        o.write_line(|o| o.write("fn serialize(&self, buf: &mut Vec<u8>) {"));
        o.with_indent(|o| {
            for f in fields {
                o.write_line(|o| {
                    o.write("self.");
                    o.write(&f.name);
                    o.write(".serialize(buf);");
                });
            }
        });
        o.write_line(|o| o.write("}"));
    });
    output.write_line(|o| o.write("}"));

    output.write("\n");

    output.write_line(|o| {
        o.write("impl crate::Deserialize for ");
        o.write(&name);
        o.write(" {");
    });
    output.with_indent(|o| {
        o.write_line(|o| o.write("fn deserialize(cur: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::deserialize::Error> {"));
        o.with_indent(|o| {
            for f in fields {
                o.write_line(|o| {
                    o.write("let ");
                    o.write(&f.name);
                    o.write(" = ");
                    f.typ.generate(o);
                    o.write("::deserialize(cur)?;");
                });
            }
            o.write("\n");
            o.write_line(|o| {
                o.write("Ok(Self { ");
                for f in fields {
                    o.write(&f.name);
                    o.write(", ");
                }
                o.write("})");
            });
        });
        o.write_line(|o| o.write("}"));
    });
    output.write_line(|o| o.write("}"));

    if let Some(typ) = typ {
        output.write("\n");

        output.write_line(|o| {
            o.write("impl crate::Function for ");
            o.write(&name);
            o.write(" {");
        });
        output.with_indent(|o| {
            o.write_line(|o| {
                o.write("type Return = ");
                typ.generate(o);
                o.write(";");
            });
        });
        output.write_line(|o| o.write("}"));
    }
}
