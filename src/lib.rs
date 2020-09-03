use std::collections::HashMap;


#[derive(Debug, Clone)]
pub enum EncoderType {
    Ordinal,
    OneHot,
    CustomMapping
}


#[derive(Debug)]
pub struct Encoder {
    pub enctype: EncoderType,
    pub mapping: HashMap<String, u64>,
}

impl Encoder {
    pub fn new(enctype: Option<EncoderType>) -> Self {
        let enctype = enctype.unwrap_or(EncoderType::Ordinal);

        Encoder {
            mapping: HashMap::new(),
            enctype: enctype
        }
    }

    /// Fit label encoder given the type (ordinal, one-hot, custom)
    /// 
    pub fn fit(&self, data: &Vec<String>) -> Self {
        let mut mapping: HashMap<String, u64> = HashMap::new();

        match self.enctype {
            EncoderType::Ordinal => {
                let mut current_idx = 0u64;

                for el in data.iter() {
                    if !mapping.contains_key(el) {
                        mapping.insert(el.to_string(), current_idx);
                        current_idx += 1;
                    }
                }
            },

            EncoderType::OneHot => unimplemented!(),

            EncoderType::CustomMapping => unimplemented!()
        }

        Encoder {
            mapping: mapping,
            enctype: self.enctype.clone(),
        }
    }

    /// Transform data to normalized encoding
    ///
    pub fn transform(&self, data: &Vec<String>) -> Vec<u64> {
        data.iter()
            .filter_map(|el| self.mapping.get(el))
            .cloned()
            .collect()
    }

    /// Transform labels back to the original data
    ///
    pub fn inverse_transform(&self, data: &Vec<u64>) -> Vec<String> {
        data.iter()
            .filter_map(|el| {
                self.mapping.iter().find_map(|(k, &v)| if v == *el { Some(k) } else { None })
            })
            .cloned()
            .collect()
    }

    /// Return the unique labels
    pub fn uniques(&self) -> Vec<String> {
        self.mapping.keys().cloned().collect()
    }

    pub fn nclasses(&self) -> usize {
        self.mapping.len()
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder_fit_and_transform() {
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
        dbg!(&enc);

        // transform original data to internal encoded representation
        let trans_data = enc.transform(&data);
        dbg!("trans data: ", &trans_data);

        let recon_data = enc.inverse_transform(&trans_data);
        dbg!("recon data: ", &recon_data);

        let uniques = enc.uniques();
        dbg!("Uniques: ", &uniques);
        assert_eq!(uniques.len(), 4);
    }

}
