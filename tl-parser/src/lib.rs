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
}

#[derive(Debug)]
pub struct Schema {
    pub types: Vec<TypeDefinition>,
    pub functions: Vec<FunctionDefinition>,
}

#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub id: u16,
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    pub id: u16,
    pub name: String,
    pub args: Vec<Field>,
    pub typ: Type,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub typ: Type,
}

#[derive(Debug, Clone)]
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

    let (types, functions) = parse_definitions(schema)?;

    Ok(Schema { types, functions })
}

fn parse_definitions<'a>(
    schema: impl Iterator<Item = (usize, &'a str)>
) -> Result<(Vec<TypeDefinition>, Vec<FunctionDefinition>), ParseError> {
    let mut types = Vec::new();
    let mut functions = Vec::new();

    let mut id = 0;

    for (idx, def) in schema {
        let line = idx + 1;
        let mut def = def.split(" ");

        match def.next() {
            Some("" | "#") => continue,
            Some("type") => types.push(parse_type_definition(id, line, def, &types)?),
            Some("func") => functions.push(parse_function_definition(id, line, def, &types, &functions)?),
            Some(typ) => return Err(ParseError::InvalidDefinitionType { line, desc: typ.to_string() }),
            None => return Err(ParseError::DefinitionTypeMissing { line }),
        };

        id = id.checked_add(1).unwrap();
    }

    Ok((types, functions))
}

fn parse_type_definition<'a>(
    id: u16,
    line: usize,
    mut def: impl Iterator<Item = &'a str>,
    definitions: &[TypeDefinition],
) -> Result<TypeDefinition, ParseError> {
    let name = def.next()
        .ok_or(ParseError::DefinitionNameMissing { line })?
        .to_string();
    if type_defined(&name, definitions) {
        return Err(ParseError::DuplicateDefinition { line, desc: name });
    }

    let fields = parse_fields(line, def, definitions)?;

    Ok(TypeDefinition { id, name, fields })
}

fn parse_function_definition<'a>(
    id: u16,
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

    let typ = def.next()
        .ok_or(ParseError::FunctionTypeMissing { line })?;
    let typ = parse_type(line, "<return>", typ, type_definitions, None)?;

    let args = parse_fields(line, def, type_definitions)?;

    Ok(FunctionDefinition { id, name, args, typ })
}

fn parse_fields<'a>(
    line: usize,
    def: impl Iterator<Item = &'a str>,
    type_definitions: &[TypeDefinition],
) -> Result<Vec<Field>, ParseError> {
    let mut fields = Vec::new();

    for part in def {
        let mut part = part.split(":");

        let name = part.next()
            .ok_or(ParseError::FieldNameMissing { line })?
            .to_string();
        if field_defined(&name, &fields) {
            return Err(ParseError::DuplicateField { line, desc: name });
        }

        let typ = part.next()
            .ok_or(ParseError::FieldTypeMissing { line })?;
        let typ = parse_type(line, &name, typ, type_definitions, None)?;

        fields.push(Field { name, typ });
    }

    Ok(fields)
}

fn parse_type(
    line: usize,
    field: &str,
    typ: &str,
    definitions: &[TypeDefinition],
    outer: Option<OuterType>,
) -> Result<Type, ParseError> {
    let typ = match typ {
        "int32" => Type::Int32,
        "int64" => Type::Int64,
        "float" => Type::Float,
        "bool" => Type::Bool,
        "string" => Type::String,
        "bytes" => Type::Bytes,
        "time" => Type::Time,
        _ if typ.starts_with('[') && typ.ends_with(']') =>
            Type::Vector(Box::new(parse_type(
                line,
                field,
                &typ[1..typ.len() - 1],
                definitions,
                Some(OuterType::Vector),
            )?)),
        _ if typ.ends_with('?') =>
            Type::Option(Box::new(parse_type(
                line,
                field,
                &typ[..typ.len() - 1],
                definitions,
                Some(OuterType::Option),
            )?)),
        _ if type_defined(typ, definitions) => Type::Defined(typ.to_string()),
        _ => return Err(ParseError::InvalidType { line, field: field.to_string(), desc: typ.to_string() }),
    };

    if let Some(outer) = outer {
        if matches!(
            (&outer, &typ),
            (OuterType::Vector, Type::Option(_))
            | (OuterType::Option, Type::Bool)
            | (OuterType::Option, Type::Vector(_))
            | (OuterType::Option, Type::Option(_))
        ) {
            return Err(ParseError::InvalidType {
                line,
                field: field.to_string(),
                desc: format!("nonsense nested type: {:?}<{:?}>", outer, typ),
            });
        }
    }

    Ok(typ)
}

fn type_defined(name: &str, definitions: &[TypeDefinition]) -> bool {
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
