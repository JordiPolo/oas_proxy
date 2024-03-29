use openapiv3::*;

/// Extension methods for `ParamSchema`
pub trait ParameterSchemaOrContentExt {
    fn item(&self) -> &Schema;
}

impl ParameterSchemaOrContentExt for ParameterSchemaOrContent {
    fn item(&self) -> &Schema {
        match self {
            ParameterSchemaOrContent::Schema(reference) => match reference {
                ReferenceOr::Reference { .. } => {
                    unimplemented!("References inside schemas are not supported")
                }
                ReferenceOr::Item(item) => item,
            },
            ParameterSchemaOrContent::Content(_content) => {
                unimplemented!("Not quite understand what's a Content in Schema")
            }
        }
    }
}

/// This extension deals with `ReferenceOr` enums in the spec
/// All these methods assume the spec has been previously dereferenced,
/// see the `deref_all` method in the spec object.
/// These methods are still needed because structs hold `ReferenceOr` enums, although
/// these enums always have an item, never a reference.
pub trait ReferenceOrExt<T> {

    /// Returns mutable reference to internal Item for a `ReferenceOr`
    fn to_item_mut(&mut self) -> &mut T;
}

impl<T> ReferenceOrExt<T> for ReferenceOr<T> {
    /// # Examples
    ///
    /// ```
    /// let mut item = ReferenceOr::Item(3);
    /// assert_eq!(to_item_mut(&mut item), &3);
    /// ```
    fn to_item_mut(&mut self) -> &mut T {

        match self {
            ReferenceOr::Reference { reference } => {
                unimplemented!("No support to dereference {}.", reference)
            }
            ReferenceOr::Item(item) => item,
        }
    }
}
