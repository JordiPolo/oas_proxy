use openapiv3::*;
use crate::error::DerefError;

pub fn deref_all(mut spec: OpenAPI) -> OpenAPI {
    let ein_spec = spec.clone(); //hack
    for (_, path_item) in &mut spec.paths {
        let p_item = deref_mut(path_item);
        deref_all_params(&mut p_item.parameters, &ein_spec);

        let p_item2 = p_item.clone(); // hack as things are used, etc.

        for operation in operation_list(p_item) {
            deref_all_params(&mut operation.parameters, &ein_spec);
            // Move from path level params to each operation so it is easier later
            for param in &p_item2.parameters {
                operation.parameters.push(param.clone());
            }
        }
    }
    spec
}

pub fn deref_own<T>(the_ref: ReferenceOr<T>) -> T {
    match the_ref {
        ReferenceOr::Reference { reference } => {
            unimplemented!("No support to dereference {}.", reference)
        }
        ReferenceOr::Item(item) => item,
    }
}

pub fn deref<T>(the_ref: &ReferenceOr<T>) -> &T {
    match the_ref {
        ReferenceOr::Reference { reference } => {
            unimplemented!("No support to dereference {}.", reference)
        }
        ReferenceOr::Item(item) => item,
    }
}

pub fn deref_mut<T>(the_ref: &mut ReferenceOr<T>) -> &mut T {
    match the_ref {
        ReferenceOr::Reference { reference } => {
            unimplemented!("No support to dereference {}.", reference)
        }
        ReferenceOr::Item(item) => item,
    }
}



fn deref_all_params(parameters: &mut Vec<ReferenceOr<Parameter>>, spec: &OpenAPI) {
    for parameter in parameters.iter_mut() {
        match parameter {
            ReferenceOr::Reference { reference } => {
                // TODO: Reference should be moved here
                let p =
                    find_parameter_reference(&spec, &reference).expect("No Param Reference found!");
                *parameter = ReferenceOr::Item(p);
            }
            ReferenceOr::Item(_) => {}
        }
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


fn find_parameter_reference(spec: &OpenAPI, reference: &str) -> Result<Parameter, DerefError> {
    //debug!("Searching for reference {}", reference);
    let reference_name: &str = reference.rsplit('/').nth(0).unwrap();
    // let reference_category: &str = reference.rsplit('/').nth(1).unwrap();

    let components = spec
        .components
        .as_ref()
        .expect("There was a reference but components is not present in the file!");

    let parameter = components
        .parameters
        .get(reference_name)
        .ok_or(DerefError::ParamError {
            name: reference_name.to_string(),
        })?;

    match parameter {
        ReferenceOr::Reference { reference: _ } => {
            unimplemented!("Reference in parameters are not supported")
        }
        ReferenceOr::Item(item) => Ok(item.clone()),
    }
}


