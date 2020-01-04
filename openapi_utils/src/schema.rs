use openapiv3::*;

/// Schema methods
pub trait SchemaExt {
    /// Returns the type of the schema
    /// Panics for oneOf, anyOf, allOf
    fn get_type(&self) -> &Type;
}

impl SchemaExt for Schema {
    fn get_type(&self) -> &Type {
        match &self.schema_kind {
            SchemaKind::Type(schema_type) => schema_type,
            SchemaKind::OneOf { .. } => unimplemented!("OneOf not supported"),
            SchemaKind::AnyOf { .. } => unimplemented!("AnyOf not supported"),
            SchemaKind::AllOf { .. } => unimplemented!("AllOf not supported"),
            SchemaKind::Any(_) => unimplemented!("Any not supported"),
        }
    }
}
