# Openapi Utils

Extensions methods for multiple structs of the [Openapiv3 library](https://github.com/glademiller/openapiv3).


This library provides a `deref_all` method on the root `openapiv3::OpenAPI` data type. This method would inline all the "$ref" in the document.

```
use openapi_utils::SpecExt;

pub fn read<P: AsRef<Path>>(filename: P) -> openapiv3::OpenAPI {
    let data = std::fs::read_to_string(filename).expect("OpenAPI file could not be read.");
    serde_yaml::from_str(&data).expect("Could not deserialize file as OpenAPI v3.0 yaml");
}

let spec = read(filename).deref_all();
```


The provided `deref` method in the ReferenceOr structure does not really do any dereference, it assumes the structures have been previously dereferenced. `deref` is still useful to make your code shorter by just choosing always the deref item instead of checking for references at every spot.

For other method in other structures, please read the documentation of the method.

To use these methods you need to `use` the extension structure for the corresponding structure of Openapiv3

