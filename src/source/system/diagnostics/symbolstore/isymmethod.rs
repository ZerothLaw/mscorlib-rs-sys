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
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

use system::diagnostics::symbolstore::ISymbolNamespace;
use system::diagnostics::symbolstore::ISymbolDocument;
use system::diagnostics::symbolstore::ISymbolScope;
use system::diagnostics::symbolstore::SymbolToken;

RIDL!{#[uuid(0x25c72eb0, 0xe437, 0x3f17, 0x94, 0x6d, 0x3b, 0x72, 0xa3, 0xac, 0xff, 0x37)]
interface ISymbolMethod(ISymbolMethodVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Token(
        pRetVal: SymbolToken,
    ) -> HRESULT,
    fn get_SequencePointCount(
        pRetVal: c_long,
    ) -> HRESULT,
    fn GetSequencePoints(
        offsets: *mut SAFEARRAY, //int[] 
        documents: *mut SAFEARRAY, //ISymbolDocument[]
        lines: *mut SAFEARRAY, //int[]
        columns: *mut SAFEARRAY, //int[]
        endLines: *mut SAFEARRAY,//int[]
        endColumns: *mut SAFEARRAY,//int[]
    ) -> HRESULT, 
    fn get_RootScope(
		pRetVal: *mut *mut  ISymbolScope ,
	) -> HRESULT,
    fn GetScope(
        offset: c_long, 
        pRetVal: *mut *mut ISymbolScope,
    ) -> HRESULT,
    fn GetOffset(
        document: *mut ISymbolDocument, 
        line: c_long, 
        column: c_long, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn GetRanges(
        document: *mut ISymbolDocument, 
        line: c_long, 
        column: c_long, 
        pRetVal: *mut *mut SAFEARRAY ,  //int[]
    ) -> HRESULT,
    fn GetParameters(
		pRetVal: *mut *mut SAFEARRAY , //ISymbolVariable[]
	) -> HRESULT,
    fn GetNamespace(
		pRetVal: *mut *mut  ISymbolNamespace ,
	) -> HRESULT,
    fn GetSourceStartEnd(
        docs: *mut SAFEARRAY, //ISymbolDocument[]
        lines: *mut SAFEARRAY, //int[]
        columns: *mut SAFEARRAY,  //int[]
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}
