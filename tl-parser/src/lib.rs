use aws_lc_rs::digest;
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("line {line}: definition type is missing")]
    DefinitionTypeMissing { line: usize },

    #[error("line {line}: invalid definition type: {desc}")]
    InvalidDefinitionType { line: usize, desc: String },

    #[error("line {line}: definition name is missing")]
    DefinitionNameMissing { line: usize },

    #[error("line {line}: duplicate definition: {desc}")]
    DuplicateDefinition { line: usize, desc: String },

    #[error("line {line}: function type is missing")]
    FunctionTypeMissing { line: usize },

    #[error("line {line}: field name is missing")]
    FieldNameMissing { line: usize },

    #[error("line {line}: duplicate field: {desc}")]
    DuplicateField { line: usize, desc: String },

    #[error("line {line}: field type is missing")]
    FieldTypeMissing { line: usize },

    #[error("line {line}: field {field}: invalid type: {desc}")]
    InvalidType { line: usize, field: String, desc: String },

    #[error("line {line}: enum is missing")]
    EnumMissing { line: usize },
}

pub struct Schema {
    pub types: Vec<TypeDefinition>,
    pub errors: Vec<ErrorDefinition>,
    pub functions: Vec<FunctionDefinition>,
}

pub struct TypeDefinition {
    pub id: u32,
    pub name: String,
    pub fields: Vec<Field>,
    pub r#enum: String,
}

pub struct ErrorDefinition {
    pub id: u32,
    pub name: String,
    pub fields: Vec<Field>,
}

pub struct FunctionDefinition {
    pub id: u32,
    pub name: String,
    pub fields: Vec<Field>,
    pub ret: Type,
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

pub fn parse_schema(schema: &str) -> Result<Schema, ParseError> {
    let schema = schema.split("\n").enumerate();

    let (types, errors, functions) = parse_definitions(schema)?;

    Ok(Schema { types, errors, functions })
}

#[allow(clippy::type_complexity)]
fn parse_definitions<'a>(
    schema: impl Iterator<Item = (usize, &'a str)>
) -> Result<(Vec<TypeDefinition>, Vec<ErrorDefinition>, Vec<FunctionDefinition>), ParseError> {
    let mut types = Vec::new();
    let mut errors = Vec::new();
    let mut functions = Vec::new();

    for (idx, def) in schema {
        let line = idx + 1;
        let id = compute_definition_id(def);
        let mut def = def.split(" ");

        match def.next() {
            Some("" | "#") => continue,
            Some("type") => types.push(parse_type_definition(id, line, def, &types)?),
            Some("error") => errors.push(parse_error_definition(id, line, def, &types, &errors)?),
            Some("func") => functions.push(parse_function_definition(id, line, def, &types, &functions)?),
            Some(r#type) => return Err(ParseError::InvalidDefinitionType { line, desc: r#type.to_string() }),
            None => return Err(ParseError::DefinitionTypeMissing { line }),
        };
    }

    Ok((types, errors, functions))
}

fn compute_definition_id(def: &str) -> u32 {
    let digest = digest::digest(&digest::SHA3_256, def.as_bytes());
    let mut buf = [0; 4];
    digest.as_ref().read_exact(&mut buf).unwrap();
    u32::from_le_bytes(buf)
}

fn parse_type_definition<'a>(
    id: u32,
    line: usize,
    mut def: impl Iterator<Item = &'a str>,
    type_definitions: &[TypeDefinition],
) -> Result<TypeDefinition, ParseError> {
    let name = def.next()
        .ok_or(ParseError::DefinitionNameMissing { line })?
        .to_string();
    if type_defined(&name, type_definitions) {
        return Err(ParseError::DuplicateDefinition { line, desc: name });
    }

    let fields = parse_fields(line, &mut def, type_definitions)?;

    let r#enum = def.next()
        .ok_or(ParseError::EnumMissing { line })?
        .to_string();

    Ok(TypeDefinition { id, name, fields, r#enum })
}

fn parse_error_definition<'a>(
    id: u32,
    line: usize,
    mut def: impl Iterator<Item = &'a str>,
    type_definitions: &[TypeDefinition],
    error_definitions: &[ErrorDefinition],
) -> Result<ErrorDefinition, ParseError> {
    let name = def.next()
        .ok_or(ParseError::DefinitionNameMissing { line })?
        .to_string();
    if error_defined(&name, error_definitions) {
        return Err(ParseError::DuplicateDefinition { line, desc: name });
    }

    let fields = parse_fields(line, &mut def, type_definitions)?;
    assert_eq!(def.next(), None);

    Ok(ErrorDefinition { id, name, fields })
}

fn parse_function_definition<'a>(
    id: u32,
    line: usize,
    mut def: impl Iterator<Item = &'a str>,
    type_definitions: &[TypeDefinition],
    function_definitions: &[FunctionDefinition],
) -> Result<FunctionDefinition, ParseError> {
    let name = def.next()
        .ok_or(ParseError::DefinitionNameMissing { line })?
        .to_string();
    if function_defined(&name, function_definitions) {
        return Err(ParseError::DuplicateDefinition { line, desc: name });
    }

    let fields = parse_fields(line, &mut def, type_definitions)?;

    let ret = def.next()
        .ok_or(ParseError::FunctionTypeMissing { line })?;
    let ret = parse_type(line, "<return>", ret, type_definitions, None)?;

    Ok(FunctionDefinition { id, name, fields, ret })
}

fn parse_fields<'a>(
    line: usize,
    def: &mut impl Iterator<Item = &'a str>,
    type_definitions: &[TypeDefinition],
) -> Result<Vec<Field>, ParseError> {
    let mut fields = Vec::new();

    for part in def {
        if part == "=" {
            break;
        }

        let mut part = part.split(":");

        let name = part.next()
            .ok_or(ParseError::FieldNameMissing { line })?
            .to_string();
        if field_defined(&name, &fields) {
            return Err(ParseError::DuplicateField { line, desc: name });
        }

        let r#type = part.next()
            .ok_or(ParseError::FieldTypeMissing { line })?;
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
) -> Result<Type, ParseError> {
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
        _ if enum_defined(r#type, type_definitions) => Type::Defined(r#type.to_string()),
        _ => return Err(ParseError::InvalidType { line, field: field.to_string(), desc: r#type.to_string() }),
    };

    if let Some(outer) = outer {
        if matches!(
            (&outer, &r#type),
            (OuterType::Vector, Type::Option(_))
            | (OuterType::Option, Type::Bool)
            | (OuterType::Option, Type::Vector(_))
            | (OuterType::Option, Type::Option(_))
        ) {
            return Err(ParseError::InvalidType {
                line,
                field: field.to_string(),
                desc: format!("invalid nested type: {:?}<{:?}>", outer, r#type),
            });
        }
    }

    Ok(r#type)
}

fn type_defined(name: &str, definitions: &[TypeDefinition]) -> bool {
    definitions.iter()
        .any(|def| def.name == name)
}

fn error_defined(name: &str, definitions: &[ErrorDefinition]) -> bool {
    definitions.iter()
        .any(|def| def.name == name)
}

fn function_defined(name: &str, definitions: &[FunctionDefinition]) -> bool {
    definitions.iter()
        .any(|def| def.name == name)
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
