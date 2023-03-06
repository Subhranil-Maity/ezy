use std::collections::HashMap;

use crate::{EzyValue, EzyKeyValuePair};

pub struct EzyKeyValuePairBuilder {
    pair: HashMap<String, EzyValue>,
    last_updated: i64
}
impl EzyKeyValuePairBuilder {
    pub fn new() -> Self {
        EzyKeyValuePairBuilder { pair: HashMap::new(), last_updated: 0 }
    }
}
impl EzyKeyValuePairBuilder {
    pub fn build(self) -> EzyKeyValuePair{
        EzyKeyValuePair { pairs: self.pair, last_updated: self.last_updated }
    }
    pub fn parse(&self, buffer: &mut Vec<u8>) {
        

        *buffer = Vec::new()
    }
}
