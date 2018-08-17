//    Copyright 2018 Tyler Laing
// 
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
// 
//        http://www.apache.org/licenses/LICENSE-2.0
// 
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

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
