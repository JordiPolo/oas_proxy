use crate::error::DerefError;
use openapiv3::*;
use indexmap::IndexMap;

/// to_item_* functions return the item of a reference without searching
/// These functions will panic if they find a reference. They
/// Are used as convenience methods in a document already dereferenced.
///
/// # Examples
///
/// ```
/// let item = ReferenceOr::Item(3);
/// assert_eq!(to_item(item), 3);
/// ```
pub fn to_item<T>(the_ref: ReferenceOr<T>) -> T {
    match the_ref {
        ReferenceOr::Reference { reference } => {
            unimplemented!("No support to dereference {}.", reference)
        }
        ReferenceOr::Item(item) => item,
    }
}

/// # Examples
///
/// ```
/// let item = ReferenceOr::Item(3);
/// assert_eq!(to_item_ref(&item), &3);
/// ```
pub fn to_item_ref<T>(the_ref: &ReferenceOr<T>) -> &T {
    match the_ref {
        ReferenceOr::Reference { reference } => {
            unimplemented!("No support to dereference {}.", reference)
        }
        ReferenceOr::Item(item) => item,
    }
}

/// # Examples
///
/// ```
/// let mut item = ReferenceOr::Item(3);
/// assert_eq!(to_item_mut(&mut item), &3);
/// ```
pub fn to_item_mut<T>(the_ref: &mut ReferenceOr<T>) -> &mut T {
    match the_ref {
        ReferenceOr::Reference { reference } => {
            unimplemented!("No support to dereference {}.", reference)
        }
        ReferenceOr::Item(item) => item,
    }
}

/// Dereferences all the internal json references in a document by copying
/// the items in the place of the references.
pub fn deref_all(mut spec: OpenAPI) -> OpenAPI {
    let components = spec
    .components
    .as_ref()
    .expect("Dereferenciation needs `components` to be present in the file.");

    for (_, path_item) in &mut spec.paths {
        deref_everything_in_path(path_item, &components);
    }
    println!("{:?}", spec);
    spec
}


fn deref_everything_in_path(path_item: &mut ReferenceOr<PathItem>, components: &Components) {

    let p_item = to_item_mut(path_item); //TODO: no Deref possible? Where in components?
    set_deref_all_params(&mut p_item.parameters, &components);

    let p_item2 = p_item.clone(); // hack as things are used, etc.

    for operation in operation_list(p_item) {
        // inline params
        set_deref_all_params(&mut operation.parameters, &components);
        // Move from path level params to each operation so it is easier later
        for param in &p_item2.parameters {
            operation.parameters.push(param.clone());
        }

        // inline request body
        if operation.request_body.is_some() {
            let mut req_body = operation.request_body.as_mut().unwrap();
            set_deref(&mut req_body, &components.request_bodies);
            let body: &mut RequestBody = to_item_mut(req_body);
            for (_, media) in &mut body.content {
                let mut schema = media.schema.as_mut().unwrap();
                set_deref(&mut schema, &components.schemas);
                set_defer_schema_contents(to_item_mut(schema), components, 10);
            }
        }

        // inline responses
        for (_status_code, response) in &mut operation.responses.responses {
            set_deref(response, &components.responses);
            for (_name, header) in &mut to_item_mut(response).headers {
                set_deref(header, &components.headers);
            }
            for (_, media) in &mut to_item_mut(response).content {
                let mut schema = media.schema.as_mut().unwrap();
                set_deref(&mut schema, &components.schemas);
                set_defer_schema_contents(to_item_mut(schema), components, 10);
            }
        }

    }
}

fn set_defer_schema_contents(schema: &mut Schema, components: &Components, recursion: i32) {
    if recursion == 0 {
        return;
    }
    match &mut schema.schema_kind {
        SchemaKind::Type(schema_type) => {
            match schema_type {
                Type::Object(object) => {
                    for (_name, mut property) in &mut object.properties {
                        set_deref_box(&mut property, &components.schemas);
                        set_defer_schema_contents(&mut to_item_mut(&mut property), components, recursion - 1);
                    }
                },
                Type::Array(array) => {
                    set_deref_box(&mut array.items, &components.schemas);
                    set_defer_schema_contents(&mut to_item_mut(&mut array.items), components, recursion - 1);
                },
                _ => { return; }
            }
        }

        SchemaKind::OneOf { ref mut one_of } => {
            for mut sch in &mut one_of.iter_mut() {
                set_deref(&mut sch, &components.schemas);
                set_defer_schema_contents(&mut to_item_mut(&mut sch), components, recursion -1 );
            }
        },
        SchemaKind::AnyOf { ref mut any_of } => {
            for mut sch in &mut any_of.iter_mut() {
                set_deref(&mut sch, &components.schemas);
                set_defer_schema_contents(&mut to_item_mut(&mut sch), components, recursion -1 );
            }
        },
        SchemaKind::AllOf { ref mut all_of } => {
            for mut sch in &mut all_of.iter_mut() {
                set_deref(&mut sch, &components.schemas);
                set_defer_schema_contents(&mut to_item_mut(&mut sch), components, recursion -1 );
            }
        },
        SchemaKind::Any(schema) => {
            for (_name, mut property) in &mut schema.properties {
                set_deref_box(&mut property, &components.schemas);
                set_defer_schema_contents(&mut to_item_mut(&mut property), components, recursion - 1);
            }
            if schema.items.is_some() {
                let mut the_items = schema.items.as_mut().unwrap();
                set_deref_box(&mut the_items, &components.schemas);
                set_defer_schema_contents(&mut to_item_mut(&mut the_items), components, recursion - 1);
            }
        }
    }
}


fn set_deref_all_params(parameters: &mut Vec<ReferenceOr<Parameter>>, components: &Components) {
    for parameter in parameters.iter_mut() {
        set_deref(parameter, &components.parameters);
    }
}


fn set_deref<'a, T>(item: &'a mut ReferenceOr<T>, component_type: &IndexMap<String, ReferenceOr<T>>) where T:Clone {
    match item {
        ReferenceOr::Reference { reference } => {
            let p = find_reference(reference, component_type).expect("No Reference found!");
            *item = ReferenceOr::Item(p);
        }
        ReferenceOr::Item(_) => {  }
    }
}

fn set_deref_box<'a, T>(item: &'a mut ReferenceOr<Box<T>>, component_type: &IndexMap<String, ReferenceOr<T>>) where T:Clone {
    match item {
        ReferenceOr::Reference { reference } => {
            let p = find_reference(reference, component_type).expect("No Reference found!");
            *item = ReferenceOr::Item(Box::new(p));
        }
        ReferenceOr::Item(_) => {  }
    }
}

fn find_reference<T>(reference: &str, component_type: &IndexMap<String, ReferenceOr<T>>) -> Result<T, DerefError> where T:Clone {
    let reference_name: &str = reference.rsplit('/').nth(0).unwrap();

    let ref_item = component_type
        .get(reference_name)
        .ok_or(DerefError::ReferenceError {
            name: reference_name.to_string(),
        })?;

    match ref_item {
        ReferenceOr::Reference { reference: _ } => {
            unimplemented!("References in references are not supported")
        }
        ReferenceOr::Item(item) => Ok(item.clone()),
    }
}


fn operation_list<'a>(item: &'a mut PathItem) -> Vec<&'a mut Operation> {
    let mut result = Vec::new();
    let mut pusher = |operation: &'a mut Option<Operation>| {
        if operation.is_some() {
            result.push(operation.as_mut().unwrap());
        }
    };
    pusher(&mut item.delete);
    pusher(&mut item.get);
    pusher(&mut item.head);
    pusher(&mut item.options);
    pusher(&mut item.patch);
    pusher(&mut item.post);
    pusher(&mut item.put);
    pusher(&mut item.trace);
    result
}
