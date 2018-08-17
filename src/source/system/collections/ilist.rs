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

use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

RIDL!{#[uuid(0x7bcfa00f, 0xf764, 0x3113, 0x91, 0x40, 0x3b, 0xbd, 0x12, 0x7a, 0x96, 0xbb)]
interface IList(IListVtbl) : IDispatch(IDispatchVtbl){
    fn get_Item(
        index: c_long, 
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn putref_Item(
        index: c_long, 
        pRetVal: VARIANT, 
    ) -> HRESULT,
    fn Add(
        value: VARIANT, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn Contains(
        value: VARIANT, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn get_IsReadOnly(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_IsFixedSize(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn IndexOf(
        value: VARIANT, 
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn Insert(
        index: c_long, 
        value: VARIANT, 
    ) -> HRESULT,
    fn Remove(
        value: VARIANT, 
    ) -> HRESULT,
    fn RemoveAt(
        index: c_long,
    ) -> HRESULT,
}} //inherits from ICollection