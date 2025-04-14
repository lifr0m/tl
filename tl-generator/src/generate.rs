use crate::Output;
use convert_case::{Case, Casing};
use std::collections::HashMap;
use tl_parser::*;

pub(crate) fn generate_schema(
    o: &mut Output,
    schema: &Schema,
) {
    generate_enum(
        o, "Error", false,
        &schema.errors.iter()
            .map(|def| &def.core)
            .collect::<Vec<_>>(),
    );

    o.write("\n");

    generate_enum(
        o, "Function", true,
        &schema.functions.iter()
            .map(|def| &def.core)
            .collect::<Vec<_>>(),
    );

    o.write("\n");

    o.write_line(|o| o.write("pub mod types {"));
    o.with_indent(|o| {
        let mut enums = HashMap::new();
        for def in &schema.types {
            enums.entry(def.r#enum.clone()).or_insert_with(Vec::new).push(&def.core);
        }
        for (name, definitions) in enums {
            generate_enum(o, &name, false, &definitions);
            o.write("\n");
        }
    });
    o.write_line(|o| o.write("}"));

    o.write("\n");

    o.write_line(|o| o.write("pub mod functions {"));
    o.with_indent(|o| {
        for def in &schema.functions {
            generate_definition(o, &def.core, Some(&def.ret));
            o.write("\n");
        }
    });
    o.write_line(|o| o.write("}"));
}

fn get_definition_name(
    def: &DefinitionCore,
    is_function: bool,
) -> String {
    if is_function {
        def.name.to_case(Case::Pascal)
    } else {
        def.name.clone()
    }
}

fn generate_definition_id(
    o: &mut Output,
    def: &DefinitionCore,
) {
    o.write(&def.id.to_string());
    o.write("_u32");
}

fn generate_type(
    o: &mut Output,
    typ: &Type,
    in_mod: bool,
) {
    match typ {
        Type::Int32 => o.write("i32"),
        Type::Int64 => o.write("i64"),
        Type::Float => o.write("f64"),
        Type::Bool => o.write("bool"),
        Type::String => o.write("String"),
        Type::Bytes => o.write("Vec::<u8>"),
        Type::Time => o.write("std::time::SystemTime"),
        Type::Vector(typ) => {
            o.write("Vec::<");
            generate_type(o, typ, in_mod);
            o.write(">");
        }
        Type::Option(typ) => {
            o.write("Option::<");
            generate_type(o, typ, in_mod);
            o.write(">");
        }
        Type::Defined(name) => {
            if in_mod {
                o.write("super::");
            }
            o.write("types::");
            o.write(name);
        }
    };
}

fn generate_enum(
    o: &mut Output,
    name: &str,
    is_function: bool,
    definitions: &[&DefinitionCore],
) {
    o.write_line(|o| o.write("#[derive(Debug, Clone, PartialEq)]"));
    o.write_line(|o| {
        o.write("pub enum ");
        o.write(name);
        o.write(" {");
    });
    o.with_indent(|o| {
        for &def in definitions {
            if is_function {
                o.write_line(|o| {
                    o.write(&get_definition_name(def, is_function));
                    o.write("(self::functions::");
                    o.write(&get_definition_name(def, is_function));
                    o.write("),");
                });
            } else {
                o.write_line(|o| {
                    o.write(&get_definition_name(def, is_function));
                    o.write(" {");
                });
                o.with_indent(|o| {
                    for field in &def.fields {
                        o.write_line(|o| {
                            o.write(&field.name);
                            o.write(": ");
                            generate_type(o, &field.typ, false);
                            o.write(",");
                        });
                    }
                });
                o.write_line(|o| o.write("},"));
            }
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
            o.write_line(|o| o.write("fn serialize(&self, dst: &mut Vec<u8>) {"));
            o.with_indent(|o| {
                o.write_line(|o| o.write("match self {"));
                o.with_indent(|o| {
                    for &def in definitions {
                        o.write_line(|o| {
                            o.write("Self::");
                            o.write(&get_definition_name(def, is_function));
                            o.write(" { ");
                            for field in &def.fields {
                                o.write(&field.name);
                                o.write(": ");
                                o.write(&field.name);
                                o.write("_, ");
                            }
                            o.write("} => {");
                        });
                        o.with_indent(|o| {
                            o.write_line(|o| {
                                generate_definition_id(o, def);
                                o.write(".serialize(dst);");
                            });
                            for field in &def.fields {
                                o.write_line(|o| {
                                    o.write(&field.name);
                                    o.write("_.serialize(dst);");
                                });
                            }
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
        o.write_line(|o| o.write("fn deserialize(src: &mut &[u8]) -> Result<Self, crate::deserialize::Error> {"));
        o.with_indent(|o| {
            o.write_line(|o| o.write("let id = u32::deserialize(src)?;"));
            o.write("\n");
            o.write_line(|o| o.write("Ok(match id {"));
            o.with_indent(|o| {
                for &def in definitions {
                    if is_function {
                        o.write_line(|o| {
                            generate_definition_id(o, def);
                            o.write(" => Self::");
                            o.write(&get_definition_name(def, is_function));
                            o.write("(self::functions::");
                            o.write(&get_definition_name(def, is_function));
                            o.write("::deserialize(src)?),");
                        });
                    } else {
                        o.write_line(|o| {
                            generate_definition_id(o, def);
                            o.write(" => {");
                        });
                        o.with_indent(|o| {
                            for field in &def.fields {
                                o.write_line(|o| {
                                    o.write("let ");
                                    o.write(&field.name);
                                    o.write("_ = ");
                                    generate_type(o, &field.typ, false);
                                    o.write("::deserialize(src)?;");
                                });
                            }
                            o.write("\n");
                            o.write_line(|o| {
                                o.write("Self::");
                                o.write(&get_definition_name(def, is_function));
                                o.write(" { ");
                                for field in &def.fields {
                                    o.write(&field.name);
                                    o.write(": ");
                                    o.write(&field.name);
                                    o.write("_, ");
                                }
                                o.write("}");
                            });
                        });
                        o.write_line(|o| o.write("}"));
                    }
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
    def: &DefinitionCore,
    ret: Option<&Type>,
) {
    o.write_line(|o| o.write("#[derive(Debug, Clone, PartialEq)]"));
    o.write_line(|o| {
        o.write("pub struct ");
        o.write(&get_definition_name(def, ret.is_some()));
        o.write(" {");
    });
    o.with_indent(|o| {
        for field in &def.fields {
            o.write_line(|o| {
                o.write("pub ");
                o.write(&field.name);
                o.write(": ");
                generate_type(o, &field.typ, true);
                o.write(",");
            });
        }
    });
    o.write_line(|o| o.write("}"));

    o.write("\n");

    o.write_line(|o| {
        o.write("impl crate::Serialize for ");
        o.write(&get_definition_name(def, ret.is_some()));
        o.write(" {");
    });
    o.with_indent(|o| {
        o.write_line(|o| o.write("fn serialize(&self, dst: &mut Vec<u8>) {"));
        o.with_indent(|o| {
            if ret.is_some() {
                o.write_line(|o| {
                    generate_definition_id(o, def);
                    o.write(".serialize(dst);");
                });
            }
            for field in &def.fields {
                o.write_line(|o| {
                    o.write("self.");
                    o.write(&field.name);
                    o.write(".serialize(dst);");
                });
            }
        });
        o.write_line(|o| o.write("}"));
    });
    o.write_line(|o| o.write("}"));

    o.write("\n");

    o.write_line(|o| {
        o.write("impl crate::Deserialize for ");
        o.write(&get_definition_name(def, ret.is_some()));
        o.write(" {");
    });
    o.with_indent(|o| {
        o.write_line(|o| o.write("fn deserialize(src: &mut &[u8]) -> Result<Self, crate::deserialize::Error> {"));
        o.with_indent(|o| {
            for field in &def.fields {
                o.write_line(|o| {
                    o.write("let ");
                    o.write(&field.name);
                    o.write("_ = ");
                    generate_type(o, &field.typ, true);
                    o.write("::deserialize(src)?;");
                });
            }
            o.write("\n");
            o.write_line(|o| {
                o.write("Ok(Self { ");
                for field in &def.fields {
                    o.write(&field.name);
                    o.write(": ");
                    o.write(&field.name);
                    o.write("_, ");
                }
                o.write("})");
            });
        });
        o.write_line(|o| o.write("}"));
    });
    o.write_line(|o| o.write("}"));

    if let Some(ret) = ret {
        o.write("\n");

        o.write_line(|o| {
            o.write("impl crate::Call for ");
            o.write(&get_definition_name(def, true));
            o.write(" {");
        });
        o.with_indent(|o| {
            o.write_line(|o| {
                o.write("type Return = ");
                generate_type(o, ret, true);
                o.write(";");
            });
        });
        o.write_line(|o| o.write("}"));
    }
}
