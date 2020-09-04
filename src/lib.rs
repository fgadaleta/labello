use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum EncoderType {
    // encode categorical features with an ordinal encoding
    Ordinal,
    // encode categorical features as one-hot numeric array
    OneHot,
    // TODO
    CustomMapping
}

/// Transformed data type
///
#[derive(Debug, Clone)]
pub enum Transform {
    Ordinal(Vec<u64>),
    OneHot(Vec<String>),
}

#[derive(Debug)]
pub enum Encoder2<T>
where T: Debug + Eq + Hash
 {
    Ordinal(HashMap<T, u64>),
    OneHot(HashMap<T, String>),
    CustomMapping(HashMap<T, String>)
}

impl <T> Encoder2<T>
where T: Debug + Eq + Hash + Clone
{
    pub fn new(enctype: Option<EncoderType>) -> Encoder2<T> {
        let enctype = enctype.unwrap_or(EncoderType::Ordinal);

        match enctype {
            EncoderType::Ordinal => Encoder2::Ordinal(HashMap::new()),
            _ => Encoder2::Ordinal(HashMap::new())
        }
    }

    /// Fit label encoder given the type (ordinal, one-hot, custom)
    ///
    pub fn fit(&mut self, data: &Vec<T>) {
        match self {
            Encoder2::Ordinal(map) => {
                let mut current_idx = 0u64;
                for el in data.iter() {
                    if !map.contains_key(el) {
                        map.insert(el.clone(), current_idx);
                        current_idx +=1;
                    }
                }
            },
            _ => unimplemented!(),

        }
    }

    /// Transform data to normalized encoding
    ///
    pub fn transform(&self, data: &Vec<T>) -> Transform {
        match self {
            Encoder2::Ordinal(map) => {
                        // for each element in data, get the value at mapping[element]
                        let res: Vec<u64> = data.iter()
                            .filter_map(|el| map.get(el))
                            .cloned()
                            .collect();
                            Transform::Ordinal(res)
                    },
            _ => unimplemented!()
        }
    }

    /// Transform labels back to original encoding.
    ///
    pub fn inverse_transform(&self, data: &Transform) -> Transform {
        match self {
            Encoder2::Ordinal(map) => {
                match data {
                    Transform::Ordinal(typed_data) => {
                            let map = map.clone();
                            let result = typed_data.into_iter()
                                    .filter_map(|el| map.iter()
                                            .find_map(|(key, &val)| if val == *el { Some(key) } else { None })
                                            .cloned()
                                            .collect());

                            // let something: Vec<u64> = result.into_iter();
                            Transform::Ordinal(result)
                    },

                    Transform::OneHot(t) => panic!("Transformed data not compatible with this encode");
                }
            },

            Encoder2::OneHot(map) => unimplemented!(),

            Encoder2::CustomMapping(map) => unimplemented!(),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fit_data() {
        let data: Vec<String> = vec!["hello".to_string(),
                                     "world".to_string(),
                                     "world".to_string(),
                                     "world".to_string(),
                                     "world".to_string(),
                                     "again".to_string(),
                                     "hello".to_string(),
                                     "again".to_string(),
                                     "goodbye".to_string()];

        let mut enc: Encoder2<String> = Encoder2::Ordinal(HashMap::new());
        dbg!("encoder: ", &enc);
        let _ = enc.fit(&data);
        // dbg!(enc.mapping.clone());
        // assert_eq!(enc.nclasses(), 4);
        let trans_data = enc.transform(&data);
        dbg!(trans_data.clone());
        // let recon_data = enc.inverse_transform(&trans_data);
        // dbg!(recon_data.clone());
        // assert_eq!(recon_data.len(), 9);
        // let uniques = enc.uniques();
        // dbg!("uniques: ", uniques);
    }
}
