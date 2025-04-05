use aws_lc_rs::digest;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("line {line}: definition type is missing")]
    DefinitionTypeMissing { line: usize },

    #[error("line {line}: invalid definition type")]
    InvalidDefinitionType { line: usize },

    #[error("line {line}: definition name is missing")]
    DefinitionNameMissing { line: usize },

    #[error("line {line}: duplicate definition")]
    DuplicateDefinition { line: usize },

    #[error("line {line}: function type is missing")]
    FunctionTypeMissing { line: usize },

    #[error("line {line}: field {field}: name is missing")]
    FieldNameMissing { line: usize, field: usize },

    #[error("line {line}: duplicate field: {field}")]
    DuplicateField { line: usize, field: String },

    #[error("line {line}: field {field}: type is missing")]
    FieldTypeMissing { line: usize, field: String },

    #[error("line {line}: field {field}: invalid type: {type}")]
    InvalidType { line: usize, field: String, r#type: String },

    #[error("line {line}: enum is missing")]
    EnumMissing { line: usize },
}

pub struct Schema {
    pub types: Vec<TypeDefinition>,
    pub errors: Vec<ErrorDefinition>,
    pub functions: Vec<FunctionDefinition>,
}

pub struct DefinitionCore {
    pub id: u32,
    pub name: String,
    pub fields: Vec<Field>,
}

pub struct TypeDefinition {
    pub core: DefinitionCore,
    pub r#enum: String,
}

pub struct ErrorDefinition {
    pub core: DefinitionCore,
}

pub struct FunctionDefinition {
    pub core: DefinitionCore,
    pub r#return: Type,
}

pub struct Field {
    pub name: String,
    pub r#type: Type,
}

#[derive(Debug)]
pub enum Type {
    Int32,
    Int64,
    Float,
    Bool,
    String,
    Bytes,
    Time,
    Vector(Box<Type>),
    Option(Box<Type>),
    Defined(String),
}

#[derive(Debug)]
enum OuterType {
    Vector,
    Option,
}

pub fn parse_schema(schema: &str) -> Result<Schema, Error> {
    let mut types = Vec::new();
    let mut errors = Vec::new();
    let mut functions = Vec::new();

    for (idx, def) in schema.split("\n").enumerate() {
        let line = idx + 1;
        let id = compute_definition_id(def);
        let mut def = def.split(" ");

        match def.next() {
            Some("" | "#") => continue,
            Some("type") => types.push(parse_type_definition(id, line, def, &types)?),
            Some("error") => errors.push(parse_error_definition(id, line, def, &types, &errors)?),
            Some("func") => functions.push(parse_function_definition(id, line, def, &types, &functions)?),
            Some(_) => return Err(Error::InvalidDefinitionType { line }),
            None => return Err(Error::DefinitionTypeMissing { line }),
        };
    }

    Ok(Schema { types, errors, functions })
}

fn compute_definition_id(def: &str) -> u32 {
    let digest = digest::digest(&digest::SHA3_256, def.as_bytes());
    let mut buf = [0; 4];
    buf.clone_from_slice(&digest.as_ref()[..4]);
    u32::from_le_bytes(buf)
}

fn parse_type_definition<'a>(
    id: u32,
    line: usize,
    mut def: impl Iterator<Item = &'a str>,
    type_definitions: &[TypeDefinition],
) -> Result<TypeDefinition, Error> {
    let core = parse_definition_core(id, line, &mut def, type_definitions)?;

    if type_defined(&core.name, type_definitions) {
        return Err(Error::DuplicateDefinition { line });
    }

    let r#enum = def.next()
        .ok_or(Error::EnumMissing { line })?
        .to_owned();

    Ok(TypeDefinition { core, r#enum })
}

fn parse_error_definition<'a>(
    id: u32,
    line: usize,
    mut def: impl Iterator<Item = &'a str>,
    type_definitions: &[TypeDefinition],
    error_definitions: &[ErrorDefinition],
) -> Result<ErrorDefinition, Error> {
    let core = parse_definition_core(id, line, &mut def, type_definitions)?;

    if error_defined(&core.name, error_definitions) {
        return Err(Error::DuplicateDefinition { line });
    }

    Ok(ErrorDefinition { core })
}

fn parse_function_definition<'a>(
    id: u32,
    line: usize,
    mut def: impl Iterator<Item = &'a str>,
    type_definitions: &[TypeDefinition],
    function_definitions: &[FunctionDefinition],
) -> Result<FunctionDefinition, Error> {
    let core = parse_definition_core(id, line, &mut def, type_definitions)?;

    if function_defined(&core.name, function_definitions) {
        return Err(Error::DuplicateDefinition { line });
    }

    let r#return = def.next()
        .ok_or(Error::FunctionTypeMissing { line })?;
    let r#return = parse_type(line, "<return>", r#return, type_definitions, None)?;

    Ok(FunctionDefinition { core, r#return })
}

fn parse_definition_core<'a>(
    id: u32,
    line: usize,
    def: &mut impl Iterator<Item = &'a str>,
    type_definitions: &[TypeDefinition],
) -> Result<DefinitionCore, Error> {
    let name = def.next()
        .ok_or(Error::DefinitionNameMissing { line })?
        .to_owned();

    let fields = parse_fields(line, def, type_definitions, "=")?;

    Ok(DefinitionCore { id, name, fields })
}

fn parse_fields<'a>(
    line: usize,
    def: &mut impl Iterator<Item = &'a str>,
    type_definitions: &[TypeDefinition],
    stop: &str,
) -> Result<Vec<Field>, Error> {
    let mut fields = Vec::new();

    for (idx, part) in def.enumerate() {
        if part == stop {
            break;
        }

        let mut part = part.split(":");

        let name = part.next()
            .ok_or(Error::FieldNameMissing { line, field: idx + 1 })?
            .to_owned();
        if field_defined(&name, &fields) {
            return Err(Error::DuplicateField { line, field: name });
        }

        let r#type = part.next()
            .ok_or(Error::FieldTypeMissing { line, field: name.clone() })?;
        let r#type = parse_type(line, &name, r#type, type_definitions, None)?;

        fields.push(Field { name, r#type });
    }

    Ok(fields)
}

fn parse_type(
    line: usize,
    field: &str,
    r#type: &str,
    type_definitions: &[TypeDefinition],
    outer: Option<OuterType>,
) -> Result<Type, Error> {
    let r#type = match r#type {
        "int32" => Type::Int32,
        "int64" => Type::Int64,
        "float" => Type::Float,
        "bool" => Type::Bool,
        "string" => Type::String,
        "bytes" => Type::Bytes,
        "time" => Type::Time,
        _ if r#type.starts_with('[') && r#type.ends_with(']') =>
            Type::Vector(Box::new(parse_type(
                line,
                field,
                &r#type[1..r#type.len() - 1],
                type_definitions,
                Some(OuterType::Vector),
            )?)),
        _ if r#type.ends_with('?') =>
            Type::Option(Box::new(parse_type(
                line,
                field,
                &r#type[..r#type.len() - 1],
                type_definitions,
                Some(OuterType::Option),
            )?)),
        _ if enum_defined(r#type, type_definitions) => Type::Defined(r#type.to_owned()),
        _ => return Err(Error::InvalidType { line, field: field.to_owned(), r#type: r#type.to_owned() }),
    };

    if let Some(outer) = outer {
        if matches!(
            (&outer, &r#type),
            (OuterType::Vector, Type::Option(_))
            | (OuterType::Option, Type::Bool)
            | (OuterType::Option, Type::Vector(_))
            | (OuterType::Option, Type::Option(_))
        ) {
            return Err(Error::InvalidType {
                line,
                field: field.to_owned(),
                r#type: format!("{outer:?}<{type:?}>"),
            });
        }
    }

    Ok(r#type)
}

fn type_defined(name: &str, definitions: &[TypeDefinition]) -> bool {
    definitions.iter()
        .any(|def| def.core.name == name)
}

fn error_defined(name: &str, definitions: &[ErrorDefinition]) -> bool {
    definitions.iter()
        .any(|def| def.core.name == name)
}

fn function_defined(name: &str, definitions: &[FunctionDefinition]) -> bool {
    definitions.iter()
        .any(|def| def.core.name == name)
}

fn field_defined(name: &str, fields: &[Field]) -> bool {
    fields.iter()
        .any(|f| f.name == name)
}

fn enum_defined(name: &str, type_definitions: &[TypeDefinition]) -> bool {
    type_definitions.iter()
        .any(|def| def.r#enum == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_id() {
        let def = "type Message id:int32 text:string? photos:[bytes] sent_at:time";
        assert_eq!(compute_definition_id(def), 226668223);
    }
}
