use crate::output::Output;
use convert_case::{Case, Casing};
use std::collections::HashMap;
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

        generate_enums(output, &self.types);

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
        generate_definition(output, self.id, self.name.clone(), &self.fields, Some(&self.r#return));
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

fn collect_enums(type_definitions: &[TypeDefinition]) -> HashMap<String, Vec<&TypeDefinition>> {
    let mut enums = HashMap::new();
    for def in type_definitions {
        enums.entry(def.r#enum.clone()).or_insert_with(Vec::new).push(def);
    }
    enums
}

fn generate_enums(
    output: &mut Output,
    type_definitions: &[TypeDefinition],
) {
    output.write_line(|o| o.write("pub mod enums {"));
    
    output.with_indent(|o| {
        for (name, definitions) in collect_enums(type_definitions) {
            o.write_line(|o| o.write("#[derive(Debug)]"));
            o.write_line(|o| {
                o.write("pub enum ");
                o.write(&name);
                o.write(" {");
            });
            o.with_indent(|o| {
                for &def in &definitions {
                    o.write_line(|o| {
                        o.write(&def.name);
                        o.write("(crate::types::");
                        o.write(&def.name);
                        o.write("),");
                    });
                }
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
                    o.write_line(|o| o.write("use crate::Identify;"));
                    o.write("\n");
                    o.write_line(|o| o.write("match self {"));
                    o.with_indent(|o| {
                        for &def in &definitions {
                            o.write_line(|o| {
                                o.write("Self::");
                                o.write(&def.name);
                                o.write("(x) => {");
                            });
                            o.with_indent(|o| {
                                o.write_line(|o| {
                                    o.write("crate::types::");
                                    o.write(&def.name);
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
            
            o.write("\n");
            
            o.write_line(|o| {
                o.write("impl crate::Deserialize for ");
                o.write(&name);
                o.write(" {");
            });
            o.with_indent(|o| {
                o.write_line(|o| o.write("fn deserialize(cur: &mut std::io::Cursor<Vec<u8>>) -> Result<Self, crate::deserialize::Error> {"));
                o.with_indent(|o| {
                    o.write_line(|o| o.write("use crate::Identify;"));
                    o.write("\n");
                    o.write_line(|o| o.write("let id = u32::deserialize(cur)?;"));
                    o.write("\n");
                    o.write_line(|o| o.write("Ok(match id {"));
                    o.with_indent(|o| {
                        for &def in &definitions {
                            o.write_line(|o| {
                                o.write("crate::types::");
                                o.write(&def.name);
                                o.write("::ID => Self::");
                                o.write(&def.name);
                                o.write("(crate::types::");
                                o.write(&def.name);
                                o.write("::deserialize(cur)?),");
                            });
                        }
                        o.write_line(|o| o.write("_ => return Err(crate::deserialize::Error::UnexpectedDefinitionId(id)),"));
                    });
                    o.write_line(|o| o.write("})"));
                });
                o.write_line(|o| o.write("}"));
            });
            o.write_line(|o| o.write("}"));
            
            o.write("\n");
        }
    });
    
    output.write_line(|o| o.write("}"));
}

fn generate_definition(
    output: &mut Output,
    id: u32,
    mut name: String,
    fields: &[Field],
    r#return: Option<&Type>,
) {
    if r#return.is_some() {
        name = name.to_case(Case::Pascal);
    }

    output.write_line(|o| o.write("#[derive(Debug)]"));
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
                f.r#type.generate(o);
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
                    f.r#type.generate(o);
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

    if let Some(r#return) = r#return {
        output.write("\n");

        output.write_line(|o| {
            o.write("impl crate::Function for ");
            o.write(&name);
            o.write(" {");
        });
        output.with_indent(|o| {
            o.write_line(|o| {
                o.write("type Return = ");
                r#return.generate(o);
                o.write(";");
            });
        });
        output.write_line(|o| o.write("}"));
    }
}
