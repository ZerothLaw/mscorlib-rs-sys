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
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

use system::diagnostics::symbolstore::ISymbolDocumentWriter;
use system::diagnostics::symbolstore::SymbolToken;
use system::diagnostics::symbolstore::SymAddressKind;
use system::intptr::IntPtr;

use system::reflection::FieldAttributes;
use system::reflection::ParameterAttributes;

RIDL!{#[uuid(0xda295a1b, 0xc5bd, 0x3b34, 0x8a, 0xcd, 0x1d, 0x7d, 0x33, 0x4f, 0xfb, 0x7f)]
interface ISymbolWriter(ISymbolWriterVtbl) : IDispatch(IDispatchVtbl) {
    fn Initialize(
        emitter: IntPtr, 
        filename: BSTR, 
        fFullBuild: VARIANT_BOOL,
    ) -> HRESULT,
    fn DefineDocument(
        url: BSTR, 
        Language: GUID, 
        LanguageVendor: GUID, 
        DocumentType: GUID, 
        pRetVal: *mut *mut ISymbolDocumentWriter,
    ) -> HRESULT,
    fn SetUserEntryPoint(
        entryMethod: *mut SymbolToken,
    ) -> HRESULT,
    fn OpenMethod(
        Method: *mut SymbolToken,
    ) -> HRESULT, 
    fn CloseMethod() -> HRESULT,
    fn DefineSequencePoints(
        document: *mut ISymbolDocumentWriter,
        offsets: *mut SAFEARRAY, //int[]
        lines: *mut SAFEARRAY, //int[]
        columns: *mut SAFEARRAY, //int[]
        endLine: *mut SAFEARRAY, //int[]
        endColumns: *mut SAFEARRAY,//int[]
    ) -> HRESULT,
    fn OpenScope(
        StartOffset: c_long,
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn CloseScope(
        EndOffset: c_long,
    ) -> HRESULT,
    fn SetScopeRange(
        scopeID: c_long, 
        StartOffset: c_long, 
        EndOffset: c_long,
    ) -> HRESULT,
    fn DefineLocalVariable(
        name: BSTR, 
        Attributes: FieldAttributes, 
        signaure: *mut SAFEARRAY, //byte[]
        addrKind: SymAddressKind,
        addr1: c_long, 
        addr2: c_long, 
        addr3: c_long, 
        StartOffset: c_long, 
        EndOffset: c_long,
    ) -> HRESULT,
    fn DefineParameter(
        name: BSTR, 
        Attributes: ParameterAttributes, 
        sequence: c_long, 
        addrKind: SymAddressKind,
        addr1: c_long, 
        addr2: c_long,
        addr3: c_long,
    ) -> HRESULT,
    fn DefineField(
        parent: SymbolToken, 
        name: BSTR, 
        Attributes: FieldAttributes, 
        signature: *mut SAFEARRAY, //byte[] 
        addrKind: SymAddressKind, 
        addr1: c_long, 
        addr2: c_long, 
        addr3: c_long,
    ) -> HRESULT,
    fn DefineGlobalVariable(
        name: BSTR, 
        Attributes: FieldAttributes, 
        signature: *mut SAFEARRAY, //byte[]
        addrKind: SymAddressKind, 
        addr1: c_long, 
        addr2: c_long, 
        addr3: c_long,
    ) -> HRESULT,
    fn Close() -> HRESULT,
    fn SetSymAttribute(
        parent: SymbolToken, 
        name: BSTR, 
        data: *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn OpenNamespace(
        name: BSTR,
    ) -> HRESULT,
    fn CloseNamespace() -> HRESULT,
    fn UsingNamespace(
        FullName: BSTR, 
    ) -> HRESULT,
    fn SetMethodSourceRange(
        startDoc: *mut ISymbolDocumentWriter, 
        startLine: c_long, 
        startColumn: c_long, 
        endDoc: *mut ISymbolDocumentWriter, 
        endLine: c_long, 
        endColumn: c_long, 
    ) -> HRESULT,
    fn SetUnderlyingWriter(
        underlyingWriter: c_long, //IntPtr?
    ) -> HRESULT,
}}