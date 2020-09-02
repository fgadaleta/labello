use std::collections::HashMap;

// #[derive(PartialEq, Eq, Hash)]
// pub enum GenericType {
//     I(i64),
//     F(f64),
//     S(String)
// }

#[derive(Debug, Clone)]
pub struct Encoder {
    pub mapping: HashMap<String, u64>,
}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder {
            mapping: HashMap::new()
        }
    }

    pub fn fit(&mut self, data: &Vec<String>) {
        // let mut unique_classes: HashMap<String, u64> = HashMap::new();
        let mut current_idx = 0u64;
        for el in data.iter() {
            if !self.mapping.contains_key(el) {
                self.mapping.insert(el.to_string(), current_idx );
                current_idx +=1;
            }
        }
    }

    pub fn nclasses(&self) -> usize {
        self.mapping.len()
    }

    /// Return the unique labels
    pub fn uniques(&self) -> Vec<String> {
        self.mapping.keys().cloned().collect()
    }

    /// Transform data to normalized encoding
    ///
    pub fn transform(&self, data: &Vec<String>) -> Vec<u64> {
        // for each element in data, get the value at mapping[element]
        data.iter()
            .filter_map(|el| self.mapping.get(el))
            .cloned()
            .collect()
    }

    /// Transform labels back to original encoding.
    ///
    pub fn inverse_transform(&self, data: &Vec<u64>) -> Vec<String> {
        data.iter()
            .filter_map(|el| {
                self.mapping.iter()
                .find_map(|(key, &val)| if val == *el { Some(key) } else { None })
            })
            .cloned()
            .collect()

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
        let mut enc = Encoder::new();
        enc.fit(&data);
        dbg!(enc.mapping.clone());
        assert_eq!(enc.nclasses(), 4);

        let trans_data = enc.transform(&data);
        dbg!(trans_data.clone());

        let recon_data = enc.inverse_transform(&trans_data);
        dbg!(recon_data.clone());

        assert_eq!(recon_data.len(), 9);

        let uniques = enc.uniques();
        dbg!("uniques: ", uniques);
    }
}