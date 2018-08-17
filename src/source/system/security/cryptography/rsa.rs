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
use winapi::um::oaidl::SAFEARRAY;

//struct __declspec(uuid("094e9135-483d-334a-aae7-8690895ab70a"))
STRUCT!{struct RSAParameters
{
    Exponent: *mut SAFEARRAY, //all byte[]
    Modulus: *mut SAFEARRAY,
    P: *mut SAFEARRAY,
    Q: *mut SAFEARRAY,
    DP: *mut SAFEARRAY,
    DQ: *mut SAFEARRAY,
    InverseQ: *mut SAFEARRAY,
    D: *mut SAFEARRAY,
}}


RIDL!{#[uuid(0x0b3fb710, 0xa25c, 0x3310, 0x87, 0x74, 0x1c, 0xf1, 0x17, 0xf9, 0x5b, 0xd4)]
interface _RSA(_RSAVtbl): IDispatch(IDispatchVtbl)  
{}} //AsymmetricAlgorithm