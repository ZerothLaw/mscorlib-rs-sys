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

RIDL!{#[uuid(0x9f8f73a3, 0x1e99, 0x3e51, 0xa4, 0x1b, 0x17, 0x9a, 0x41, 0xdc, 0x74, 0x7c)]
interface _HostProtectionAttribute(_HostProtectionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //SystemException