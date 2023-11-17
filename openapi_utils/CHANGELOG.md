# 0.5.2
- Resolve references in references

# 0.5.1
- Use Indexmap 2.1
- Use http 1.0

# 0.5.0
- Use Indexmap 2.0

# 0.4.0
- Removed `parameter_data` use `parameter.parameter_data()` and `parameter.parameter_data_ref()`
- Removed `to_item` and `to_item_ref` from reference, use `into_item` and `as_item` instead. They are slightly less convenient, if the spec is dereferenced these options will always be Some but naming sounded too similar and confusing.
- Declaring edition 2021

# 0.3.0
- Updated to openapiv3 1.0
