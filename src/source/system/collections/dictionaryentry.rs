use std::mem::zeroed;

use winapi::um::oaidl::{VARIANT};
STRUCT!{struct DictionaryEntry {
    key: VARIANT, 
    value: VARIANT,
}}
//two properties for key and value

impl DictionaryEntry {
    pub fn new() -> DictionaryEntry {
        DictionaryEntry{key: unsafe {zeroed()}, value: unsafe{zeroed()} }
    }
    
    pub fn with(key: VARIANT, value: VARIANT) -> DictionaryEntry {
        DictionaryEntry {key: key, value: value}
    }

    pub fn key(&self) -> VARIANT {
        self.key
    }

    pub fn value(&self) -> VARIANT {
        self.value
    }

    pub fn key_mut(&mut self, key: VARIANT) {
        self.key = key;
    }

    pub fn value_mut(&mut self, value: VARIANT) {
        self.value = value;
    }
}