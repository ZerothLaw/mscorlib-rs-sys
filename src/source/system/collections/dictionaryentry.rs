use winapi::um::oaidl::{VARIANT};
STRUCT!{struct DictionaryEntry {
    key: VARIANT, 
    value: VARIANT,
}}
//two properties for key and value