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
use winapi::shared::winerror::HRESULT;
use winapi::ctypes::c_long;
use winapi::um::oaidl::VARIANT;
use system::intptr::IntPtr;

RIDL!{#[uuid(0x601cd486, 0x04bf, 0x3213, 0x9e, 0xa9, 0x06, 0xeb, 0xe4, 0x35, 0x1d, 0x74)]
interface ICustomMarshaler(ICustomMarshalerVtbl) : IDispatch(IDispatchVtbl) {
    fn MarshalNativeToManaged(
        pNativeData: IntPtr, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT, 

    fn MarshalManagedToNative(
        ManagedObj: VARIANT, 
        pRetVal: *mut IntPtr,
    ) -> HRESULT,

    fn CleanUpNativeData(
        pNativeData: IntPtr, 
    ) -> HRESULT, 

    fn CleanUpManagedData(
        ManagedObj: VARIANT, 
    ) -> HRESULT,

    fn GetNativeDataSize(
        pRetVal: *mut c_long,
    ) -> HRESULT,

}}