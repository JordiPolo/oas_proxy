use openapiv3::*;
use serde_yaml;
use std::path::Path;
use crate::spec_utils;

pub fn read_and_deref_all<P: AsRef<Path>>(filename: P) -> OpenAPI {
    deref_all(read(filename))
}

fn read<P: AsRef<Path>>(filename: P) -> OpenAPI {
    let data = std::fs::read_to_string(filename).expect("OpenAPI file could not be read.");
    let spec = serde_yaml::from_str(&data).expect("Could not deserialize file as OpenAPI v3.0 yaml");
    debug!("The openapi after parsed {:?}", spec);
    spec
}

fn deref_all(mut spec: OpenAPI) -> OpenAPI {
    let ein_spec = spec.clone(); //hack
    for (_, mut path_item) in &mut spec.paths {
        let mut p_item = defer_path_item(&mut path_item);
        deref_all_params(&mut p_item.parameters, &ein_spec);
        let mut operation = path_to_operation2(&mut p_item);
        //let mut hack_op = operation;
        deref_all_params(&mut operation.parameters, &ein_spec);
    }
    spec
}

fn defer_path_item(ref_path: &mut ReferenceOr<PathItem>) -> &mut PathItem {
    match ref_path {
        ReferenceOr::Reference { .. } => {
            unimplemented!("External description of paths not supported")
        }
        ReferenceOr::Item(item) => { item }
    }

}

fn deref_all_params(parameters: &mut Vec<ReferenceOr<Parameter>>, spec: &OpenAPI) {
    for parameter in parameters.iter_mut() {
        match parameter {
            ReferenceOr::Reference { reference } => {
                // TODO: Reference should be moved here
                let p = spec_utils::find_parameter_reference(&spec, &reference).expect("No Param Reference found!");
                *parameter = ReferenceOr::Item(p);
            }
            ReferenceOr::Item(_) => {}
        }
    }

}

///

fn path_to_operation2(item: &mut PathItem) -> &mut Operation {
      item
            .get.as_mut()
            .or(item.head.as_mut())
            .or(item.options.as_mut())
            .or(item.trace.as_mut())
            .or(item.delete.as_mut())
            .or(item.patch.as_mut())
            .or(item.post.as_mut())
            .or(item.put.as_mut()).expect("Failed to read the operation for pathItem")
}


