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
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

use crate::system::collections::ICollection;
use crate::system::collections::IDictionaryEnumerator;

RIDL!{#[uuid(0x6a6841df, 0x3287, 0x3d87, 0x80, 0x60, 0xce, 0x0b, 0x4c, 0x77, 0xd2, 0xa1)]
interface IDictionary(IDictionaryVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Item(
        key: VARIANT,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn putref_Item(
        key: VARIANT,
        pRetVal: VARIANT, //says pRetVal, but not actually a return value
    ) -> HRESULT,
    fn get_Keys(
		pRetVal: *mut *mut  ICollection ,
	) -> HRESULT,
    fn get_Values(
		pRetVal: *mut *mut  ICollection ,
	) -> HRESULT,
    fn Contains(
        key: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Add(
        key: VARIANT,
        val: VARIANT,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn get_IsReadOnly(
        pRetVal: *mut VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFixedSize(
        pRetVal: *mut VARIANT_BOOL ,
    ) -> HRESULT,
    fn GetEnumerator(
		pRetVal: *mut *mut  IDictionaryEnumerator ,
	) -> HRESULT,
    fn Remove(
        key: VARIANT,
    ) -> HRESULT,
}} //inherits ICollection