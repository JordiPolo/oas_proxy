use openapiv3::*;

/// Extension methods for Schema
pub trait SchemaExt {
    /// Returns the type of the schema
    /// Panics for oneOf, anyOf, allOf, or no type defined
    fn get_type(&self) -> &Type;

    /// Returns true if a specific type is defined, false on oneOf, anyOf, allOf, no type
    fn is_type_defined(&self) -> bool;
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

    fn is_type_defined(&self) -> bool {
        match &self.schema_kind {
            SchemaKind::Type(_) => true,
            _ => false,
        }
    }
}
