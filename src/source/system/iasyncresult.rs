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
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::VARIANT;

 use system::threading::_WaitHandle;

RIDL!{#[uuid(0x11ab34e7, 0x0176, 0x3c9e, 0x9e, 0xfe, 0x19, 0x78, 0x58, 0x40, 0x0a, 0x3d)]
interface IAsyncResult(IAsyncResultVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_IsCompleted(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_AsyncWaitHandle(
			pRetVal: *mut *mut _WaitHandle,
	) -> HRESULT,
    fn get_AsyncState(
        pRetVal: *mut *mut VARIANT,
    ) -> HRESULT,
      fn get_CompletedSynchronously(
        pRetVal: *mut *mut VARIANT_BOOL,
    ) -> HRESULT,
}}