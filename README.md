# Labello: a simple label encoder

This crate provides a simple API for encoding labels represented by vectors.
It uses a hashmap as internal data structure for classes and their mapping.

## Example

```rust

// load data in a vector
let data: Vec<String> = vec!["hello".to_string(),
                             "world".to_string(),
                             "world".to_string(),
                             "world".to_string(),
                             "world".to_string(),
                             "again".to_string(),
                             "hello".to_string(),
                             "again".to_string(),
                             "goodbye".to_string()];
// declare a label encoder
let enc = Encoder::new(None);

// fit encoder with ordinal type (default)
let enc = enc.fit(&data);

// transform original data to internal encoded representation
let trans_data = enc.transform(&data);

// inverse transform internal encoded representation to original data
let recon_data = enc.inverse_transform(&trans_data);

// get unique original elements
let uniques = enc.uniques();

```