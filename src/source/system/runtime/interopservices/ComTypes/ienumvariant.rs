// ienumvariant.rs - MIT License
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

use winapi::ctypes::c_long;

use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::{SAFEARRAY};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use crate::system::IntPtr;

RIDL!{#[uuid(0x00020404, 0x0000, 0x0000, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46)]
interface IEnumVARIANT(IEnumVARIANTVtbl): IUnknown(IUnknownVtbl){
    fn Next(
        celt: c_long, 
        rgVar: *mut *mut SAFEARRAY, //object[] MarshalAs(UnmanagedType.LPArray, SizeParamIndex=0)
        pceltFetched: IntPtr,
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn Skip(
        celt: c_long, 
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn Reset(
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn Clone_(
        pRetVal: *mut *mut IEnumVARIANT, 
    ) -> HRESULT,
}}