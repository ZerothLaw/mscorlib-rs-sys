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
use winapi::shared::winerror::HRESULT;

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::unknwnbase::IUnknown;

use system::security::policy::_ApplicationTrust;

RIDL!{#[uuid(0x427e255d, 0xaf02, 0x3b0d, 0x8c, 0xe3, 0xa2, 0xbb, 0x94, 0xba, 0x30, 0x0f)]
interface IApplicationTrustManager(IApplicationTrustManagerVtbl): IDispatch(IDispatchVtbl)  
{
    fn DetermineApplicationTrust(
        activationContext: *mut IUnknown,
        Context: *mut _TrustManagerContext,
        pRetVal: *mut *mut _ApplicationTrust,
    ) -> HRESULT,
}}

//enum __declspec(uuid("940b1725-f706-3cef-9586-0f189b117c20"))
ENUM!{enum TrustManagerUIContext {
    TrustManagerUIContext_Install = 0,
    TrustManagerUIContext_Upgrade = 1,
    TrustManagerUIContext_Run = 2,
}}

RIDL!{#[uuid(0xd89eac5e, 0x0331, 0x3fcd, 0x9c, 0x16, 0x4f, 0x1e, 0xd3, 0xfe, 0x1b, 0xe2)]
interface _TrustManagerContext(_TrustManagerContextVtbl): IDispatch(IDispatchVtbl)  
{}}