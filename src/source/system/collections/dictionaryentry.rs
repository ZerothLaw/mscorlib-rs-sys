// MIT License
// Copyright (c) 2018 Tyler Laing (ZerothLaw)

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use std::mem::zeroed;

use winapi::um::oaidl::{VARIANT};

STRUCT!{struct DictionaryEntry {
    key: VARIANT, 
    value: VARIANT,
}}

//two properties for key and value
impl DictionaryEntry {
    //!Convenience impl to work like the C# one does.
    //! Unsafe to remind user that the memory is uninitialized. 
    pub unsafe fn new() -> DictionaryEntry {
        DictionaryEntry{key: zeroed(), value: zeroed() }
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