// thread.rs - MIT License
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

use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::{WORD, UINT};
use winapi::shared::ntdef::LCID;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypesbase::LPOLESTR;
use winapi::um::oaidl::{DISPID, DISPPARAMS, EXCEPINFO, ITypeInfo, VARIANT};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

RIDL!{#[uuid(0xc281c7f1, 0x4aa9, 0x3517, 0x96, 0x1a, 0x46, 0x3c, 0xfe, 0xd5, 0x7e, 0x75)]
interface _Thread(_ThreadVtbl) : IUnknown(IUnknownVtbl){
    fn GetTypeInfoCount(
        pctinfo: *mut UINT,
    ) -> HRESULT,
    fn GetTypeInfo(
        iTInfo: UINT,
        lcid: LCID,
        ppTInfo: *mut *mut ITypeInfo,
    ) -> HRESULT,
    fn GetIDsOfNames(
        riid: REFIID,
        rgszNames: *mut LPOLESTR,
        cNames: UINT,
        lcid: LCID,
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke(
        dispIdMember: DISPID,
        riid: REFIID,
        lcid: LCID,
        wFlags: WORD,
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT,
        pExcepInfo: *mut EXCEPINFO,
        puArgErr: *mut UINT,
    ) -> HRESULT,
}}