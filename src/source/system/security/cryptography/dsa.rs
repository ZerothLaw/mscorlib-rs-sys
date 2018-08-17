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
use winapi::um::oaidl::SAFEARRAY;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//struct __declspec(uuid("0c646f46-aa27-350d-88dd-d8c920ce6c2d"))
STRUCT!{struct DSAParameters
{
    P: *mut SAFEARRAY,
    Q: *mut SAFEARRAY,
    G: *mut SAFEARRAY,
    y: *mut SAFEARRAY,
    J: *mut SAFEARRAY,
    x: *mut SAFEARRAY,
    Seed: *mut SAFEARRAY,
    Counter: c_long,
}}


RIDL!{#[uuid(0x0eb5b5e0, 0x1be6, 0x3a5f, 0x87, 0xb3, 0xe3, 0x32, 0x33, 0x42, 0xf4, 0x4e)]
interface _DSA(_DSAVtbl): IDispatch(IDispatchVtbl)  
{}} //AssymetricAlgorithm