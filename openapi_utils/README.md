# Openapi Utils

This crate provides extensions methods for multiple structs of the [Openapiv3 crate](https://github.com/glademiller/openapiv3).

It aims at making it more ergonomic to work with the information in the openapi contracts.


## Usage

This crate provides a `deref_all` method on the `openapiv3::OpenAPI` data type. This method would inline all the `$ref` in the document.


### Example

```rust
use openapi_utils::SpecExt;

pub fn read<P: AsRef<Path>>(filename: P) -> openapiv3::OpenAPI {
    let data = std::fs::read_to_string(filename).expect("OpenAPI file could not be read.");
    serde_yaml::from_str(&data).expect("Could not deserialize file as OpenAPI v3.0 yaml")
}

let spec = read(filename).deref_all();
```


The `to_item`, `to_item_ref` and `to_item_mut` methods in the ReferenceOr structure assumes `deref_all` has been called on the spec previously and will panic otherwise. These methods are really a shorthand to choose the right element in the enumeration (the item).


For other methods in other structures please refer to the documentation of each extension.
To have these methods available in your structures you need to `use` the corresponding extension.

## no_std support
This crate is compatible with no_std, although a global allocator is required.


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
