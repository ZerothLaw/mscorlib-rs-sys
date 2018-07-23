use std::mem::zeroed;

use winapi::um::oaidl::{VARIANT};
STRUCT!{struct DictionaryEntry {
    key: VARIANT, 
    value: VARIANT,
}}
//two properties for key and value

impl DictionaryEntry {
    fn new() -> DictionaryEntry {
        DictionaryEntry{key: unsafe {zeroed()}, value: unsafe{zeroed()} }
    }
    
    fn with(key: VARIANT, value: VARIANT) -> DictionaryEntry {
        DictionaryEntry {key: key, value: value}
    }


    fn key(&self) -> VARIANT {
        self.key
    }

    fn value(&self) -> VARIANT {
        self.value
    }

    fn key_mut(&mut self, key: VARIANT) {
        self.key = key;
    }

    fn value_mut(&mut self, value: VARIANT) {
        self.value = value;
    }
}