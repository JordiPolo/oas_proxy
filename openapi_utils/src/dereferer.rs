use crate::error::DerefError;
use openapiv3::*;
use indexmap::IndexMap;
use crate::reference::ReferenceOrExt;

pub trait SpecExt {
    fn deref_all(self) -> OpenAPI;
}

impl SpecExt for OpenAPI {
    /// Dereferences all the internal references in a document by copying
    /// the items in the place of the references.
    fn deref_all(mut self) -> OpenAPI {
        let components = self
        .components
        .as_ref()
        .expect("Dereferenciation needs `components` to be present in the file.");

        for (_, path_item) in &mut self.paths {
            deref_everything_in_path(path_item, &components);
        }
    //    println!("{:?}", spec);
       self
    }
}




fn deref_everything_in_path(path_item: &mut ReferenceOr<PathItem>, components: &Components) {

    let p_item = path_item.to_item_mut(); //TODO: no Deref possible? Where in components?
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
            let body: &mut RequestBody = req_body.to_item_mut();
            for (_, media) in &mut body.content {
                let mut schema = media.schema.as_mut().unwrap();
                set_deref(&mut schema, &components.schemas);
                set_defer_schema_contents(schema.to_item_mut(), components, 10);
            }
        }

        // inline responses
        for (_status_code, response) in &mut operation.responses.responses {
            set_deref(response, &components.responses);
            for (_name, header) in &mut response.to_item_mut().headers {
                set_deref(header, &components.headers);
            }
            for (_, media) in &mut response.to_item_mut().content {
                let mut schema = media.schema.as_mut().unwrap();
                set_deref(&mut schema, &components.schemas);
                set_defer_schema_contents(schema.to_item_mut(), components, 10);
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
                        set_defer_schema_contents(&mut property.to_item_mut(), components, recursion - 1);
                    }
                },
                Type::Array(array) => {
                    set_deref_box(&mut array.items, &components.schemas);
                    set_defer_schema_contents(&mut array.items.to_item_mut(), components, recursion - 1);
                },
                _ => { return; }
            }
        }

        SchemaKind::OneOf { ref mut one_of } => {
            for mut sch in &mut one_of.iter_mut() {
                set_deref(&mut sch, &components.schemas);
                set_defer_schema_contents(&mut sch.to_item_mut(), components, recursion -1 );
            }
        },
        SchemaKind::AnyOf { ref mut any_of } => {
            for mut sch in &mut any_of.iter_mut() {
                set_deref(&mut sch, &components.schemas);
                set_defer_schema_contents(&mut sch.to_item_mut(), components, recursion -1 );
            }
        },
        SchemaKind::AllOf { ref mut all_of } => {
            for mut sch in &mut all_of.iter_mut() {
                set_deref(&mut sch, &components.schemas);
                set_defer_schema_contents(&mut sch.to_item_mut(), components, recursion -1 );
            }
        },
        SchemaKind::Any(schema) => {
            for (_name, mut property) in &mut schema.properties {
                set_deref_box(&mut property, &components.schemas);
                set_defer_schema_contents(&mut property.to_item_mut(), components, recursion - 1);
            }
            if schema.items.is_some() {
                let mut the_items = schema.items.as_mut().unwrap();
                set_deref_box(&mut the_items, &components.schemas);
                set_defer_schema_contents(&mut the_items.to_item_mut(), components, recursion - 1);
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
