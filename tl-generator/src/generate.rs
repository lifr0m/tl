use crate::Output;
use convert_case::{Case, Casing};
use std::collections::HashMap;
use tl_parser::*;

pub(crate) trait Generate {
    fn generate(&self, output: &mut Output);
}

impl Generate for Schema {
    fn generate(&self, output: &mut Output) {
        generate_enum(
            output, "Error", "errors",
            &self.errors.iter()
                .map(|def| get_definition_name(&def.core, false))
                .collect::<Vec<_>>(),
            false,
        );

        output.write("\n");

        generate_enum(
            output, "Function", "functions",
            &self.functions.iter()
                .map(|def| get_definition_name(&def.core, true))
                .collect::<Vec<_>>(),
            true,
        );

        output.write("\n");

        output.write_line(|o| o.write("pub mod types {"));
        output.with_indent(|o| {
            for def in &self.types {
                def.generate(o);
                o.write("\n");
            }
        });
        output.write_line(|o| o.write("}"));

        output.write("\n");

        output.write_line(|o| o.write("pub mod enums {"));
        output.with_indent(|o| {
            let mut enums = HashMap::new();
            for def in &self.types {
                enums.entry(def.r#enum.clone()).or_insert_with(Vec::new).push(def);
            }

            for (name, definitions) in enums {
                generate_enum(
                    o, &name, "types",
                    &definitions.into_iter()
                        .map(|def| get_definition_name(&def.core, false))
                        .collect::<Vec<_>>(),
                    false,
                );
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
        generate_definition(output, &self.core, None);
    }
}

impl Generate for ErrorDefinition {
    fn generate(&self, output: &mut Output) {
        generate_definition(output, &self.core, None);
    }
}

impl Generate for FunctionDefinition {
    fn generate(&self, output: &mut Output) {
        generate_definition(output, &self.core, Some(&self.r#return));
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
                output.write("crate::enums::");
                output.write(defined);
            }
        };
    }
}

fn get_definition_name(
    core: &DefinitionCore,
    is_function: bool,
) -> String {
    if is_function {
        core.name.to_case(Case::Pascal)
    } else {
        core.name.clone()
    }
}

fn generate_enum(
    o: &mut Output,
    name: &str,
    group: &str,
    definitions: &[String],
    is_function: bool,
) {
    o.write_line(|o| o.write("#[derive(Debug, Clone, PartialEq)]"));
    o.write_line(|o| {
        o.write("pub enum ");
        o.write(name);
        o.write(" {");
    });
    o.with_indent(|o| {
        for def in definitions {
            o.write_line(|o| {
                o.write(def);
                o.write(&format!("(crate::{group}::"));
                o.write(def);
                o.write("),");
            });
        }
    });
    o.write_line(|o| o.write("}"));

    if !is_function {
        o.write("\n");

        o.write_line(|o| {
            o.write("impl crate::Serialize for ");
            o.write(name);
            o.write(" {");
        });
        o.with_indent(|o| {
            o.write_line(|o| o.write("fn serialize(&self, buf: &mut Vec<u8>) {"));
            o.with_indent(|o| {
                o.write_line(|o| o.write("use crate::Identify;"));
                o.write("\n");
                o.write_line(|o| o.write("match self {"));
                o.with_indent(|o| {
                    for def in definitions {
                        o.write_line(|o| {
                            o.write("Self::");
                            o.write(def);
                            o.write("(x) => {");
                        });
                        o.with_indent(|o| {
                            o.write_line(|o| {
                                o.write(&format!("crate::{group}::"));
                                o.write(def);
                                o.write("::ID.serialize(buf);");
                            });
                            o.write_line(|o| o.write("x.serialize(buf);"));
                        });
                        o.write_line(|o| o.write("}"));
                    }
                });
                o.write_line(|o| o.write("};"));
            });
            o.write_line(|o| o.write("}"));
        });
        o.write_line(|o| o.write("}"));
    }

    o.write("\n");

    o.write_line(|o| {
        o.write("impl crate::Deserialize for ");
        o.write(name);
        o.write(" {");
    });
    o.with_indent(|o| {
        o.write_line(|o| o.write("fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {"));
        o.with_indent(|o| {
            o.write_line(|o| o.write("use crate::Identify;"));
            o.write("\n");
            o.write_line(|o| o.write("let id = u32::deserialize(reader)?;"));
            o.write("\n");
            o.write_line(|o| o.write("Ok(match id {"));
            o.with_indent(|o| {
                for def in definitions {
                    o.write_line(|o| {
                        o.write(&format!("crate::{group}::"));
                        o.write(def);
                        o.write("::ID => Self::");
                        o.write(def);
                        o.write(&format!("(crate::{group}::"));
                        o.write(def);
                        o.write("::deserialize(reader)?),");
                    });
                }
                o.write_line(|o| o.write("_ => return Err(crate::deserialize::Error::UnexpectedDefinitionId(id)),"));
            });
            o.write_line(|o| o.write("})"));
        });
        o.write_line(|o| o.write("}"));
    });
    o.write_line(|o| o.write("}"));
}

fn generate_definition(
    o: &mut Output,
    core: &DefinitionCore,
    r#return: Option<&Type>,
) {
    let name = get_definition_name(core, r#return.is_some());

    o.write_line(|o| o.write("#[derive(Debug, Clone, PartialEq)]"));
    o.write_line(|o| {
        o.write("pub struct ");
        o.write(&name);
        o.write(" {");
    });
    o.with_indent(|o| {
        for f in &core.fields {
            o.write_line(|o| {
                o.write("pub ");
                o.write(&f.name);
                o.write(": ");
                f.r#type.generate(o);
                o.write(",");
            });
        }
    });
    o.write_line(|o| o.write("}"));

    o.write("\n");

    o.write_line(|o| {
        o.write("impl crate::Identify for ");
        o.write(&name);
        o.write(" {");
    });
    o.with_indent(|o| {
        o.write_line(|o| {
            o.write("const ID: u32 = ");
            o.write(&core.id.to_string());
            o.write(";");
        });
    });
    o.write_line(|o| o.write("}"));

    o.write("\n");

    o.write_line(|o| {
        o.write("impl crate::Serialize for ");
        o.write(&name);
        o.write(" {");
    });
    o.with_indent(|o| {
        o.write_line(|o| o.write("fn serialize(&self, buf: &mut Vec<u8>) {"));
        o.with_indent(|o| {
            if r#return.is_some() {
                o.write_line(|o| o.write("use crate::Identify;"));
                o.write("\n");
                o.write_line(|o| o.write("Self::ID.serialize(buf);"));
                o.write("\n");
            }
            for f in &core.fields {
                o.write_line(|o| {
                    o.write("self.");
                    o.write(&f.name);
                    o.write(".serialize(buf);");
                });
            }
        });
        o.write_line(|o| o.write("}"));
    });
    o.write_line(|o| o.write("}"));

    o.write("\n");

    o.write_line(|o| {
        o.write("impl crate::Deserialize for ");
        o.write(&name);
        o.write(" {");
    });
    o.with_indent(|o| {
        o.write_line(|o| o.write("fn deserialize(reader: &mut crate::Reader) -> Result<Self, crate::deserialize::Error> {"));
        o.with_indent(|o| {
            for f in &core.fields {
                o.write_line(|o| {
                    o.write("let ");
                    o.write(&f.name);
                    o.write(" = ");
                    f.r#type.generate(o);
                    o.write("::deserialize(reader)?;");
                });
            }
            o.write("\n");
            o.write_line(|o| {
                o.write("Ok(Self { ");
                for f in &core.fields {
                    o.write(&f.name);
                    o.write(", ");
                }
                o.write("})");
            });
        });
        o.write_line(|o| o.write("}"));
    });
    o.write_line(|o| o.write("}"));

    if let Some(r#return) = r#return {
        o.write("\n");

        o.write_line(|o| {
            o.write("impl crate::Call for ");
            o.write(&name);
            o.write(" {");
        });
        o.with_indent(|o| {
            o.write_line(|o| {
                o.write("type Return = ");
                r#return.generate(o);
                o.write(";");
            });
        });
        o.write_line(|o| o.write("}"));
    }
}
