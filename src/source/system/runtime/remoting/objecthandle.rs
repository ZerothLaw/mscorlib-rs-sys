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

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::wtypes::BSTR;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::VARIANT;
use winapi::ctypes::c_long;
use system::reflection::_Type;
use system::runtime::remoting::objref::_ObjRef;

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