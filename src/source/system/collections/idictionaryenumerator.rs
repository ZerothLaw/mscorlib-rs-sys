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
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

use crate::system::collections::DictionaryEntry;

RIDL!{#[uuid(0x35d574bf, 0x7a4f, 0x3588, 0x8c, 0x19, 0x12, 0x21, 0x2a, 0x0f, 0xe4, 0xdc)]
interface IDictionaryEnumerator(IDictionaryEnumeratorVtbl) : IDispatch(IDispatchVtbl){
    fn get_key(
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn get_val(
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn get_Entry(
        pRetVal: *mut DictionaryEntry,
    ) -> HRESULT,
}} //inherits from IEnumerator