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
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

use crate::system::diagnostics::symbolstore::ISymbolReader;

//TODO: add DEPRECATED_RIDL macro?
RIDL!{#[uuid(0x20808adc, 0xcc01, 0x3f3a, 0x8f, 0x09, 0xed, 0x12, 0x94, 0x0f, 0xc2, 0x12)]
interface ISymbolBinder(ISymbolBinderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetReader(
        importer: c_long, 
        filename: BSTR, 
        searchPath: BSTR, 
        pRetVal: *mut *mut ISymbolReader, 
    ) -> HRESULT,
}}//obsolete

RIDL!{#[uuid(0x027c036a, 0x4052, 0x3821, 0x85, 0xde, 0xb5, 0x33, 0x19, 0xdf, 0x12, 0x11)]
interface ISymbolBinder1(ISymbolBinder1Vtbl): IDispatch(IDispatchVtbl)  
{
    fn GetReader(
        importer: c_long, 
        filename: BSTR, 
        searchPath: BSTR, 
        pRetVal: *mut *mut ISymbolReader, 
    ) -> HRESULT,
}}