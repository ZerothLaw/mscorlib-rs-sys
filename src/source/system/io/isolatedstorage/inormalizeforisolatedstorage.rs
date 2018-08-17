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

use winapi::shared::ntdef::{HRESULT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

RIDL!{#[uuid(0xf5006531, 0xd4d7, 0x319e, 0x9e, 0xda, 0x9b, 0x4b, 0x65, 0xad, 0x8d, 0x4f)]
interface INormalizeForIsolatedStorage(INormalizeForIsolatedStorageVtbl): IDispatch(IDispatchVtbl)  
{
    fn Normalize(
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
}}