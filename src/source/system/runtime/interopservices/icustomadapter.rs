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
use winapi::shared::ntdef::{HRESULT};
use winapi::um::unknwnbase::{IUnknown};

RIDL!{#[uuid(0x3cc86595, 0xfeb5, 0x3ce9, 0xba, 0x14, 0xd0, 0x5c, 0x8d, 0xc3, 0x32, 0x1c)]
interface ICustomAdapter(ICustomAdapterVtbl) : IDispatch(IDispatchVtbl)
{
    fn GetUnderlyingObject(
        pRetVal: *mut *mut IUnknown,
    ) -> HRESULT,
}}
