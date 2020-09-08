# Labello: a fast label encoder in Rust

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

// define type of encoder and configuration for fitting
let enctype = EncoderType::Ordinal;
let config = Config{
            max_nclasses: Some(3),
            mapping_function: None
};
// create encoder of <enctype>
let mut enc: Encoder<String> = Encoder::new(Some(enctype));

// fit encoder with this configuration
enc.fit(&data, &config);

// transform original data to internal encoded representation
let trans_data = enc.transform(&data);

// inverse transform internal encoded representation to original data
let recon_data = enc.inverse_transform(&trans_data);

// get unique original elements
let uniques = enc.uniques();
```