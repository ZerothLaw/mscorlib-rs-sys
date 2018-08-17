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
use winapi::shared::guiddef::GUID;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

use system::diagnostics::symbolstore::SymbolToken;
use system::diagnostics::symbolstore::ISymbolMethod;
use system::diagnostics::symbolstore::ISymbolDocument;

RIDL!{#[uuid(0xe809a5f1, 0xd3d7, 0x3144, 0x9b, 0xef, 0xfe, 0x8a, 0xc0, 0x36, 0x46, 0x99)]
interface ISymbolReader(ISymbolReaderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetDocument(
        Url: BSTR,
        Language: GUID, 
        LanguageVendor: GUID, 
        DocumentType: GUID, 
        pRetVal: *mut *mut ISymbolDocument, 
    ) -> HRESULT,
    fn GetDocuments(
		pRetVal: *mut *mut SAFEARRAY, //ISymbolDocument[]
	) -> HRESULT,
    fn get_UserEntryPoint(
        pRetVal: *mut SymbolToken,
    ) -> HRESULT,
    fn GetMethod(
        Method: SymbolToken, 
        pRetVal: *mut *mut ISymbolMethod,
    ) -> HRESULT,
    fn GetMethod_2(
        Method: SymbolToken, 
        Version: c_long,
    ) -> HRESULT,
    fn GetVariables(
        parent: SymbolToken, 
        pRetVal: *mut *mut SAFEARRAY, //ISymbolVariable[]
    ) -> HRESULT,
    fn GetGlobalVariables(
		pRetVal: *mut *mut SAFEARRAY, //ISymbolVariable[]
	) -> HRESULT,
    fn GetMethodFromDocumentPosition(
        document: *mut ISymbolDocument, 
        line: c_long, 
        column: c_long, 
        pRetVal: *mut *mut ISymbolMethod,
    ) -> HRESULT, 
    fn GetSymAttribute(
        parent: SymbolToken, 
        name: BSTR, 
        pRetVal: *mut *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn GetNamespaces(
		pRetVal: *mut *mut SAFEARRAY, //ISymbolNamespace[]
	) -> HRESULT,
}}