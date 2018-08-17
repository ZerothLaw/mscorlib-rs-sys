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

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//struct __declspec(uuid("a2959123-2f66-35b4-815d-37c83360809b"))
STRUCT!{struct NativeOverlapped {
    InternalLow: c_long,
    InternalHigh: c_long,
    OffsetLow: c_long,
    OffsetHigh: c_long,
    EventHandle: c_long,
}}

RIDL!{#[uuid(0xdd846fcc, 0x8d04, 0x3665, 0x81, 0xb6, 0xaa, 0xcb, 0xe9, 0x9c, 0x19, 0xc3)]
interface _Overlapped(_OverlappedVtbl): IDispatch(IDispatchVtbl)  
{}} //