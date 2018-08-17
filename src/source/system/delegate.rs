// delegate.rs - MIT License
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

use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY, VARIANT};

use system::runtime::serialization::{StreamingContext,_SerializationInfo};
use system::reflection::{_MethodInfo,_Type};

RIDL!{#[uuid(0xfb6ab00f, 0x5096, 0x3af8, 0xa3, 0x3d, 0xd7, 0x88, 0x5a, 0x5f, 0xa8, 0x29)]
interface _Delegate(_DelegateVtbl): IDispatch(IDispatchVtbl)  
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
    fn GetInvocationList(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn Clone_(
        pRetVal: VARIANT ,
    ) -> HRESULT,
    fn GetObjectData(
        info: *mut _SerializationInfo, 
        Context: StreamingContext,
    ) -> HRESULT,
    fn DynamicInvoke(
		args: *mut SAFEARRAY ,
		pRetVal: *mut VARIANT ,
	) -> HRESULT,
    fn get_Method(
		pRetVal: *mut *mut  _MethodInfo ,
	) -> HRESULT,
    fn get_Target(
        pRetVal: VARIANT ,
    ) -> HRESULT,
}}//implements ICloneable, ISerializable