use openapiv3::*;
use serde_yaml;
use std::path::Path;

use crate::error::DerefError;

pub fn read_and_deref_all<P: AsRef<Path>>(filename: P) -> OpenAPI {
    deref_all(read(filename))
}

pub fn read<P: AsRef<Path>>(filename: P) -> OpenAPI {
    let data = std::fs::read_to_string(filename).expect("OpenAPI file could not be read.");
    let spec =
        serde_yaml::from_str(&data).expect("Could not deserialize file as OpenAPI v3.0 yaml");
    debug!("The openapi after parsed {:?}", spec);
    spec
}

pub fn deref_all(mut spec: OpenAPI) -> OpenAPI {
    let ein_spec = spec.clone(); //hack
    for (_, mut path_item) in &mut spec.paths {
        let mut p_item = deref(&mut path_item);
        deref_all_params(&mut p_item.parameters, &ein_spec);
        let operation = path_to_operation(&mut p_item);
        deref_all_params(&mut operation.parameters, &ein_spec);
    }
    spec
}


pub fn deref<T>(the_ref: &mut ReferenceOr<T>) -> &mut T {
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

///

fn path_to_operation(item: &mut PathItem) -> &mut Operation {
    item.get
        .as_mut()
        .or(item.head.as_mut())
        .or(item.options.as_mut())
        .or(item.trace.as_mut())
        .or(item.delete.as_mut())
        .or(item.patch.as_mut())
        .or(item.post.as_mut())
        .or(item.put.as_mut())
        .expect("Failed to read the operation for pathItem")
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


