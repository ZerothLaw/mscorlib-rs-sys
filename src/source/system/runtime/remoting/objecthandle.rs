// objecthandle.rs - MIT License
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
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

use crate::system::reflection::_Type;
use crate::system::runtime::remoting::_ObjRef;

RIDL!{#[uuid(0xea675b47, 0x64e0, 0x3b5f, 0x9b, 0xe7, 0xf7, 0xdc, 0x29, 0x90, 0x73, 0x0d)]
interface _ObjectHandle(_ObjectHandleVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        obj: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
      fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
    fn GetLifetimeService(
        pRetVal: VARIANT ,
    ) -> HRESULT,
    fn InitializeLifetimeService(
        pRetVal: VARIANT ,
    ) -> HRESULT,
    fn CreateObjRef(
		requestedType: *mut  _Type ,
		pRetVal: *mut *mut  _ObjRef ,
	) -> HRESULT,
    fn Unwrap(
        pRetVal: VARIANT,
    ) -> HRESULT,
}}