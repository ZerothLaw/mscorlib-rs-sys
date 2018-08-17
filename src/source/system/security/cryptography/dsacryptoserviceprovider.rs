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

RIDL!{#[uuid(0x1f38aafe, 0x7502, 0x332f, 0x97, 0x1f, 0xc2, 0xfc, 0x70, 0x0a, 0x1d, 0x55)]
interface _DSACryptoServiceProvider(_DSACryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}} //implements DSA, ICspAssymetricAlgorithm