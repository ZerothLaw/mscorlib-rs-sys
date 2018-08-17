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
use winapi::shared::guiddef::GUID;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR,VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

RIDL!{#[uuid(0x1c32f012, 0x2684, 0x3efe, 0x8d, 0x50, 0x9c, 0x29, 0x73, 0xac, 0xc0, 0x0b)]
interface ISymbolDocument(ISymbolDocumentVtbl) : IDispatch(IDispatchVtbl) {
    fn get_Url(
        pRetVal: *mut BSTR, 
    ) -> HRESULT,
    fn get_DocumentType(
        pRetVal: *mut GUID, 
    ) -> HRESULT,
    fn get_Language(
        pRetVal: *mut GUID,
    ) -> HRESULT,
    fn get_LanguageVendor(
        pRetVal: *mut GUID, 
    ) -> HRESULT,
    fn get_CheckSumAlgorithmId(
        pRetVal: *mut GUID,
    ) -> HRESULT,
    fn GetCheckSum(
        pRetVal: *mut *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn FindClosestLine(
        line: c_long, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_HasEmbeddedSource(
        pRetVal: *mut VARIANT_BOOL, 
    ) -> HRESULT,
    fn get_SourceLength(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn GetSourceRange(
        startLine: c_long, 
        startColumn: c_long, 
        endLine: c_long, 
        endColumn: c_long,
        pRetVal: *mut *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
}} 