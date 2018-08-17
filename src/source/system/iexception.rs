// iexception.rs - MIT License
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
use winapi::shared::wtypes::{BSTR,VARIANT_BOOL};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

use system::reflection::{_MethodBase,_Type};
use system::runtime::serialization::StreamingContext;
use system::runtime::serialization::_SerializationInfo;

RIDL!{#[uuid(0xb36b5c63, 0x42ef, 0x38bc, 0xa0, 0x7e, 0x0b, 0x34, 0xc9, 0x8f, 0x16, 0x4a)]
interface _Exception(_ExceptionVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ToString(
        pRetVal: BSTR,
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
    fn get_Message(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn GetBaseException(
		pRetVal: *mut *mut  _Exception ,
	) -> HRESULT,
    fn get_StackTrace(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_HelpLink(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn put_HelpLink(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn get_Source(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn put_Source(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn GetObjectData(
        info: *mut _SerializationInfo,
        Context: StreamingContext,
    ) -> HRESULT,
    fn get_InnerException(
		pRetVal: *mut *mut  _Exception ,
	) -> HRESULT,
    fn get_TargetSite(
		pRetVal: *mut *mut  _MethodBase ,
	) -> HRESULT,
}}
