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