use crate::error::E;
use openapiv3::*;
use hyper::Method;
use log::debug;
use serde_yaml;
use std::path::Path;

pub fn read<P: AsRef<Path>>(filename: P) -> OpenAPI {
    let data = std::fs::read_to_string(filename).expect("OpenAPI file could not be read.");
    let spec =
        serde_yaml::from_str(&data).expect("Could not deserialize file as OpenAPI v3.0 yaml");
    debug!("The openapi after parsed {:?}", spec);
    spec
}

pub fn path_to_operation<'a>(
    item: &'a mut PathItem,
    method: &Method,
) -> Result<&'a mut Operation, E> {
    debug!("item {:?}", item);
    let inner =
        |op: &'a mut Option<Operation>| op.as_mut().ok_or_else(|| E::MethodError(format!("{:?}", method)));
    match *method {
        Method::DELETE => inner(&mut item.delete),
        Method::GET => inner(&mut item.get),
        Method::HEAD => inner(&mut item.head),
        Method::OPTIONS => inner(&mut item.options),
        Method::PATCH => inner(&mut item.patch),
        Method::POST => inner(&mut item.post),
        Method::PUT => inner(&mut item.put),
        _ => unimplemented!("Method not supported"),
    }
}

pub fn operation_list(item: &PathItem) -> Vec<(&str, &Operation)> {
    let mut result = Vec::new();
    result.push(("delete", &item.delete));
    result.push(("get", &item.get));
    result.push(("head", &item.head));
    result.push(("options", &item.options));
    result.push(("patch", &item.patch));
    result.push(("post", &item.post));
    result.push(("put", &item.put));
    result
        .iter()
        .filter(|(_n, o)| o.is_some())
        .map(|(name, oper)| (*name, oper.as_ref().unwrap()))
        .collect()
}

pub fn used(description: &mut Option<String>) {
    *description = Some("1".to_string());
}
