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

RIDL!{#[uuid(0xa16537bc, 0x1edf, 0x3516, 0xb7, 0x5e, 0xcc, 0x65, 0xca, 0xf8, 0x73, 0xab)]
interface _SHA1CryptoServiceProvider(_SHA1CryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}} //SHA1