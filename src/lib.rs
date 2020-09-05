use std::cmp::Eq;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;

const fn num_bits<T>() -> usize {
    std::mem::size_of::<T>() * 8
}
fn log_2(x: i32) -> u32 {
    assert!(x > 0);
    num_bits::<i32>() as u32 - x.leading_zeros() - 1
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

/// Transformed data type
///
#[derive(Debug, Clone)]
pub enum Transform {
    Ordinal(Vec<u64>),
    OneHot(Vec<String>),
    CustomMapping(Vec<String>),
}

#[derive(Debug)]
pub enum Encoder2<T>
where
    T: Debug + Eq + Hash,
{
    Ordinal(HashMap<T, u64>),
    OneHot(HashMap<T, Vec<bool>>),
    CustomMapping(HashMap<T, String>),
}

impl<T> Encoder2<T>
where
    T: Debug + Eq + Hash + Clone,
{
    pub fn new(enctype: Option<EncoderType>) -> Encoder2<T> {
        let enctype = enctype.unwrap_or(EncoderType::Ordinal);

        match enctype {
            EncoderType::Ordinal => Encoder2::Ordinal(HashMap::new()),

            EncoderType::OneHot => Encoder2::OneHot(HashMap::new()),

            EncoderType::CustomMapping => unimplemented!(),
        }
    }

    /// Return number of unique categories
    ///
    pub fn nclasses(&self) -> usize {
        match self {
            Encoder2::Ordinal(map) => map.len(),
            Encoder2::OneHot(map) => map.len(),
            Encoder2::CustomMapping(map) => map.len(),
        }
    }

    /// Fit label encoder given the type (ordinal, one-hot, custom)
    ///
    pub fn fit(&mut self, data: &Vec<T>) {
        let datalen = data.len();
        dbg!("datalen ", datalen);

        match self {
            Encoder2::Ordinal(map) => {
                let mut current_idx = 0u64;
                for el in data.iter() {
                    if !map.contains_key(el) {
                        map.insert(el.clone(), current_idx);
                        current_idx += 1;
                    }
                }
            }

            Encoder2::OneHot(map) => {
                let mut mapping: HashMap<T, u64> = HashMap::new();
                let mut current_idx = 0u64;
                for el in data.iter() {
                    if !mapping.contains_key(el) {
                        mapping.insert(el.clone(), current_idx);
                        current_idx += 1;
                    }
                }
                // create a vector of as many elements as unique categories
                let vecsize = mapping.len();

                // TODO don't like this here
                let mut ohe_repr: Vec<bool> = Vec::with_capacity(vecsize);

                for (key, value) in mapping.into_iter() {
                    // convert value to binary
                    let mut ohe_repr: Vec<bool> = format!("{:b}", value)
                        .chars()
                        .enumerate()
                        .filter_map(|(i, n)| match n {
                            '1' => {
                                ohe_repr.push(true);
                                Some(true)
                            }
                            '0' => Some(false),
                            _ => panic!("Invalid conversion to binary"),
                        })
                        .collect();

                    // push remaining zeros (vecsize - current_len)
                    for _ in 0..vecsize - ohe_repr.len() { ohe_repr.push(false);}

                    // insert into final hashmap
                    map.insert(key, ohe_repr);
                }
            }

            Encoder2::CustomMapping(map) => unimplemented!(),
        }
    }

    /// Transform data to normalized encoding
    ///
    pub fn transform(&self, data: &Vec<T>) -> Transform {
        match self {
            Encoder2::Ordinal(map) => {
                // for each element in data, get the value at mapping[element]
                let res: Vec<u64> = data.iter().filter_map(|el| map.get(el)).cloned().collect();
                Transform::Ordinal(res)
            }

            Encoder2::OneHot(_map) => unimplemented!(),

            Encoder2::CustomMapping(_map) => unimplemented!(),
        }
    }

    /// Transform labels back to original encoding.
    ///
    pub fn inverse_transform(&self, data: &Transform) -> Vec<T> {
        match self {
            Encoder2::Ordinal(map) => match data {
                Transform::Ordinal(typed_data) => {
                    let result: Vec<T> = typed_data
                        .into_iter()
                        .flat_map(|&el| {
                            map.iter()
                                .filter(move |&(_key, val)| val == &el)
                                .map(|(key, &_val)| key.clone())
                        })
                        .collect();
                    result
                }

                Transform::OneHot(t) => panic!("Transformed data not compatible with this encode"),
                _ => unimplemented!(),
            },

            Encoder2::OneHot(_map) => unimplemented!(),

            Encoder2::CustomMapping(_map) => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_hot_encoding() {
        let x = 42u64;
        let ohe: Vec<bool> = format!("{:b}", x)
            .chars()
            .filter_map(|n| match n {
                '1' => Some(true),
                '0' => Some(false),
                _ => panic!("Conversion to binary failed"),
            })
            .collect();
        dbg!(ohe);

        // check number of bits is correct
        assert_eq!(log_2(128), 7);
    }
    #[test]
    fn test_fit_ordinal_encoder() {
        let data: Vec<String> = vec![
            "hello".to_string(),
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
        // create ordinal encoder
        let mut enc: Encoder2<String> = Encoder2::new(Some(enctype));
        dbg!("created encoder", &enc);

        enc.fit(&data);
        dbg!("fitted encoder", &enc);

        let trans_data = enc.transform(&data);
        dbg!("transformed data", &trans_data);

        let reco_data = enc.inverse_transform(&trans_data);
        dbg!("reconstructed data", reco_data);
        assert_eq!(enc.nclasses(), 4);
    }

    #[test]
    fn test_fit_one_hot_encoder() {
        let data: Vec<String> = vec![
            "hello".to_string(),
            "world".to_string(),
            "world".to_string(),
            "world".to_string(),
            "world".to_string(),
            "again".to_string(),
            "hello".to_string(),
            "again".to_string(),
            "goodbye".to_string(),
        ];
        let enctype = EncoderType::OneHot;
        let mut enc: Encoder2<String> = Encoder2::new(Some(enctype));
        dbg!("created encoder", &enc);
        enc.fit(&data);
        dbg!("fitted encoder", &enc);
    }
}
