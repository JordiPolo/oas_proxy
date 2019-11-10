use openapiv3::*;

#[derive(Clone)]
struct SimpleSpec {
    paths: Vec<SimplePath>,
}

#[derive(Clone)]
struct SimplePath {
    url: String,
    params: Vec<SimpleParam>,
    responses: Vec<SimpleResponse>,
}
#[derive(Clone)]
struct SimpleParam {
    name: String,
    place: ParamPlace,
}

#[derive(Clone)]
enum ParamPlace {
    Query,
    Path,
}

#[derive(Clone)]
struct SimpleResponse {
    code: String,
}

// pub fn spec_to_simple(spec: &OpenAPI) -> SimpleSpec{
//     let mut sspec = SimpleSpec { paths: Vec::new()};
//     for path in spec.paths {
//         sspec.paths.push(path_to_simple(&path));
//     }
//     sspec
// }

// fn path_to_simple(path: &(String, ReferenceOr<PathItem>)) -> SimplePath {
//     simple_params =
//     SimplePath {
//         url: path.0

//     }
// }
