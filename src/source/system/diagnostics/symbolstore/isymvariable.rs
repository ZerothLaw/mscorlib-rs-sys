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
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};

use crate::system::diagnostics::symbolstore::symaddresskind::SymAddressKind;

RIDL!{#[uuid(0x4042bd4d, 0xb5ab, 0x30e8, 0x91, 0x9b, 0x14, 0x91, 0x06, 0x87, 0xba, 0xae)]
interface ISymbolVariable(ISymbolVariableVtbl) : IDispatch(IDispatchVtbl) {
    fn get_name(
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn get_Attributes(
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn GetSignature(
        pRetVal: *mut *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn get_AddressKind(
        pRetVal: *mut SymAddressKind,
    ) -> HRESULT,
    fn get_AddressField1(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_AddressField2(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_AddressField3(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_StartOffset(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_EndOffset(
        pRetVal: *mut c_long,
    ) -> HRESULT,
}}