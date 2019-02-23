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
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

use crate::system::diagnostics::symbolstore::ISymbolMethod;

RIDL!{#[uuid(0x1cee3a11, 0x01ae, 0x3244, 0xa9, 0x39, 0x49, 0x72, 0xfc, 0x97, 0x03, 0xef)]
interface ISymbolScope(ISymbolScopeVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Method(
        pRetVal: *mut *mut  ISymbolMethod,
    ) -> HRESULT,
    fn get_parent(
        pRetVal: *mut *mut  ISymbolScope,
    ) -> HRESULT,
    fn GetChildren(
        pRetVal: *mut *mut SAFEARRAY, //ISymbolScope[]
    ) -> HRESULT,
    fn get_StartOffset(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn get_EndOffset(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetLocals(
        pRetVal: *mut *mut SAFEARRAY, //ISymbolVariable[]
    ) -> HRESULT,
    fn GetNamespaces(
        pRetVal: *mut *mut SAFEARRAY, //ISymbolNamespace[]
    ) -> HRESULT,
}}