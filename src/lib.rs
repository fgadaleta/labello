//! Labello: a fast label encoder in Rust
//!
//! With Labello it is possible to create different types of encoders: ordinal, one-hot, custom
//!
//! A custom encoder does not guarantee the reversibility of the mapping and inverse-mapping.
//! An inverse-mapping operation is reversible (reconstruct the original data) depending on the
//! mapping defined by the user.
//! The other types of encoding do guarantee that an inverse-mapping operation reconstruct the
//! original data losslessly
//!
//!

use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;
use std::fmt::Debug;
use std::iter::Iterator;

/// configuration for encoder (metadata)
#[derive(Debug, Clone)]
pub struct Config<T> {
    // maximum number of classes (repeat after max)
    pub max_nclasses: Option<u64>,
    // only for custom encoder (define closure and apply to the single element)
    pub mapping_function: Option<fn(T) -> u64>,
}

#[derive(Debug, Clone)]
pub enum EncoderType {
    // encode categorical features with an ordinal encoding
    Ordinal,
    // encode categorical features as one-hot numeric array
    OneHot,
    // user-defined mapping function
    CustomMapping,
}

#[derive(Debug)]
pub enum Encoder<T>
where T: Hash + Eq + Debug
{
    Ordinal(HashMap<T, u64>),
    OneHot(HashMap<T, OheRepr>),
    Custom(HashMap<T, u64>)
}

type OheRepr = Vec<bool>;

/// transformed data type
///
#[derive(Debug, Clone)]
pub enum Transform {
    Ordinal(Vec<u64>),
    OneHot(Vec<OheRepr>),
    CustomMapping(Vec<u64>)
}

impl Transform {
    pub fn len(&self) -> usize {
        match self {
            Transform::Ordinal(data) => data.len(),
            Transform::OneHot(data) => data.len(),
            Transform::CustomMapping(data) => data.len()
        }
    }
}

impl <T> Encoder<T>
where T: Hash + Eq + Clone + Debug
{
    pub fn new(enctype: Option<EncoderType>) -> Encoder<T> {
        let enctype = enctype.unwrap_or(EncoderType::Ordinal);

        match enctype {
            EncoderType::Ordinal => Encoder::Ordinal(HashMap::new()),
            EncoderType::OneHot => Encoder::OneHot(HashMap::new()),
            EncoderType::CustomMapping => Encoder::Custom(HashMap::new())
        }
    }

    /// Fit label encoder given the type (ordinal, one-hot, custom)
    ///
    pub fn fit(&mut self, data: &Vec<T>, config: &Config<T>) {
        let max_nclasses = config.max_nclasses.unwrap_or(u64::MAX) - 1;

        match self {
            Encoder::Ordinal(map) => {
                let mut current_idx = 0u64;
                for el in data.iter() {
                    if !map.contains_key(el) {
                        map.insert(el.clone(), current_idx);
                        if current_idx < max_nclasses {
                            current_idx += 1;
                        }
                    }
                }
            },

            Encoder::OneHot(map) => {
                let mut mapping: HashMap<T, u64> = HashMap::new();
                let mut current_idx = 0u64;
                // encode in a temporary hashmap (mapping)
                for el in data.iter() {
                    if !mapping.contains_key(el) {
                        mapping.insert(el.clone(), current_idx);
                        if current_idx < max_nclasses {
                            current_idx += 1;
                        }
                    }
                }

                let vecsize = mapping.len();
                for (key, value) in mapping.into_iter() {
                    let mut converted: OheRepr = format!("{:b}", value)
                                                .chars()
                                                .enumerate()
                                                .filter_map(|(_i, n)| match n {
                                                    '1' => {
                                                        Some(true)
                                                    },

                                                    '0' => Some(false),
                                                    _ => panic!("Invalid conversion to binary"),
                                                })
                                                .collect();
                    // push remaining zeros (vecsize - current len)
                    for _ in 0..vecsize - converted.len() {
                        converted.push(false);
                    }
                    // insert into final hashmap
                    map.insert(key, converted);
                }
            },

            Encoder::Custom(map) => {
                let mapping_func = config.mapping_function.unwrap();
                for el in data.iter() {
                    if !map.contains_key(el) {
                        let value = mapping_func(el.clone());
                        map.insert(el.clone(), value);
                    }
                }
            },
        }
    }

    /// Transform data to normalized encoding
    ///
    pub fn transform(&self, data: &Vec<T>) -> Transform  {
        match self {
            Encoder::Ordinal(map) => {
                let res: Vec<u64> = data.iter().filter_map(|el| map.get(el)).cloned().collect();
                Transform::Ordinal(res)
            }

            Encoder::OneHot(map) => {
                let res: Vec<OheRepr> = data.iter().filter_map(|el| map.get(el)).cloned().collect();
                Transform::OneHot(res)
            },

            Encoder::Custom(map) => {
                let res: Vec<u64> = data.iter().filter_map(|el| map.get(el)).cloned().collect();
                Transform::CustomMapping(res)
            },

        }

    }

    /// Transforms labels back to the original data (not necessarily true with custom encoder)
    ///
    pub fn inverse_transform(&self, data: &Transform) -> Vec<T> {
        match self {
            Encoder::Ordinal(mapping) => match data {
                Transform::Ordinal(typed_data) => {
                    let result: Vec<T> = typed_data.iter()
                    .flat_map(|&el| {
                        mapping.into_iter()
                        .filter(move |&(_key, val)| val == &el)
                        .map(|(key, &_val)| key.clone())
                    })
                    .collect();
                    result
                },
                _ => panic!("Transformed data not compatible with this encoder"),
            },

            // TODO WIP inverse mapping is not reversible for one-hot (ERROR!!)
            Encoder::OneHot(mapping) => match data {
                Transform::OneHot(typed_data) => {
                    let result: Vec<T> = typed_data.iter()
                    .flat_map(|el| {
                        mapping.into_iter()
                        .filter(move |&(_key, val)| {
                            let mut equal_el: usize = 0;
                            for i in 0..val.len() {
                                if val[i] == el[i] {
                                    equal_el += 1;
                                }
                            }
                            // val == el
                            equal_el == val.len()
                        }
                    )
                        .map(|(key, _val)| key.clone())
                    })
                    .collect();
                    result
                },
                _ => panic!("Transformed data not compatible with this encoder")
            },

            Encoder::Custom(mapping) => match data {
                Transform::CustomMapping(typed_data) => {
                    let result = typed_data.into_iter().flat_map(|&el| {
                        mapping
                            .into_iter()
                            .filter(move |&(_k, v)| v == &el)
                        .map(|(k, &_v)| k.clone())
                    })
                    .collect();
                    result
                },
                _ => panic!("Transformed data not compatible with this encoder"),
            }
        }
    }

    /// Return number of unique categories
    ///
    pub fn nclasses(&self) -> usize {
        match self {
            // TODO len is the same for every type
            Encoder::Ordinal(mapping) => {
                let values: Vec<u64> = mapping.values().cloned().collect();
                let len = values.iter().max();
                match len {
                    Some(v) => *v as usize + 1,
                    _ => 0 as usize
                }
            },
            Encoder::OneHot(map) => map.len(),
            Encoder::Custom(map) => map.len(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_hot_encoding() {
        let x = 128u64;
        let ohe: Vec<bool> = format!("{:b}", x)
            .chars()
            .filter_map(|n| match n {
                '1' => Some(true),
                '0' => Some(false),
                _ => panic!("Conversion to binary failed"),
            })
            .collect();
        dbg!(&ohe);

        assert_eq!(ohe.len(), 8);

        // check number of bits is correct
        // assert_eq!(log_2(128), 7);
    }

    #[test]
    fn test_fit_ordinal_encoder() {
        let data: Vec<String> = vec!["hello".to_string(),
                                    "world".to_string(),
                                    "world".to_string(),
                                    "world".to_string(),
                                    "world".to_string(),
                                    "again".to_string(),
                                    "hello".to_string(),
                                    "again".to_string(),
                                    "goodbye".to_string(),
                                    ];
        let enctype = EncoderType::Ordinal;
        let config = Config{
            max_nclasses: None,
            mapping_function: None
        };
        let mut enc: Encoder<String> = Encoder::new(Some(enctype));
        dbg!("created encoder ", &enc);

        enc.fit(&data, &config);
        dbg!("fitted encoder:", &enc);

        let trans_data = enc.transform(&data);
        dbg!("trans data: ", &trans_data);

        let recon_data = enc.inverse_transform(&trans_data);
        dbg!("recon data:", &recon_data);

        assert_eq!(enc.nclasses(), 4);
    }

    #[test]
    fn test_fit_ordinal_encoder_limited_classes() {
        let data: Vec<String> = vec!["hello".to_string(),
                                    "world".to_string(),
                                    "world".to_string(),
                                    "world".to_string(),
                                    "world".to_string(),
                                    "again".to_string(),
                                    "hello".to_string(),
                                    "again".to_string(),
                                    "goodbye".to_string(),
                                    ];
        let enctype = EncoderType::Ordinal;
        let config = Config{
            max_nclasses: Some(3),
            mapping_function: None
        };
        let mut enc: Encoder<String> = Encoder::new(Some(enctype));
        dbg!("created encoder ", &enc);

        enc.fit(&data, &config);
        dbg!("fitted encoder:", &enc);

        assert_eq!(enc.nclasses(), 3);
    }

    #[test]
    fn test_fit_one_hot_encoder() {
        let data: Vec<String> = vec!["hello".to_string(),
                                    "world".to_string(),
                                    "world".to_string(),
                                    "world".to_string(),
                                    "world".to_string(),
                                    "again".to_string(),
                                    "hello".to_string(),
                                    "again".to_string(),
                                    "goodbye".to_string(),
                                    ];

        let config = Config {
            max_nclasses: Some(10),
            mapping_function: None
        };
        let mut enc: Encoder<String> = Encoder::new(Some(EncoderType::OneHot));
        enc.fit(&data, &config);
        dbg!("fitted encoder: ", &enc);

        let trans_data = enc.transform(&data);
        dbg!("trans data: ", &trans_data);
        assert_eq!(trans_data.len(), data.len());

        let recon_data = enc.inverse_transform(&trans_data);
        dbg!("recon data:", &recon_data);

    }

    #[test]
    fn test_fit_custom_encoder() {
        let data: Vec<String> = vec!["hello".to_string(),
                                    "world".to_string(),
                                    "world".to_string(),
                                    "world".to_string(),
                                    "world".to_string(),
                                    "again".to_string(),
                                    "hello".to_string(),
                                    "again".to_string(),
                                    "goodbye".to_string(),
                                    ];
        let config: Config<String> = Config {
            max_nclasses: Some(10),
            mapping_function: Some(|el| match el.as_str() {
                "hello" => 42,
                "goodbye" => 99,
                _ => 0
            }),
        };

        let mut enc: Encoder<String> = Encoder::new(Some(EncoderType::CustomMapping));
        enc.fit(&data, &config);
        dbg!("fitted encoder: ", &enc);

        let trans_data = enc.transform(&data);
        dbg!("trans data: ", &trans_data);

        let recon_data = enc.inverse_transform(&trans_data);
        dbg!("recon data:", &recon_data);
    }
}