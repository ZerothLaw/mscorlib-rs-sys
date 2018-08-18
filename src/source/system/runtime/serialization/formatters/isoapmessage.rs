// isoapmessage.rs - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

RIDL!{#[uuid(0xe699146c, 0x7793, 0x3455, 0x9b, 0xef, 0x96, 0x4c, 0x90, 0xd8, 0xf9, 0x95)]
interface ISoapMessage(ISoapMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ParamNames(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_ParamNames(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_ParamValues(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_ParamValues(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_ParamTypes(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_ParamTypes(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
      fn get_MethodName(
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
      fn put_MethodName(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn get_XmlNameSpace(
        pRetVal: *mut BSTR ,
    ) -> HRESULT,
    fn put_XmlNameSpace(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn get_headers(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_headers(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
}}
