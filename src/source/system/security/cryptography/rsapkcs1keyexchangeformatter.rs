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

RIDL!{#[uuid(0x9ff67f8e, 0xa7aa, 0x3ba6, 0x90, 0xee, 0x9d, 0x44, 0xaf, 0x6e, 0x2f, 0x8c)]
interface _RSAPKCS1KeyExchangeFormatter(_RSAPKCS1KeyExchangeFormatterVtbl): IDispatch(IDispatchVtbl)  
{}} //AsymmetricKeyExchangeFormatter