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

RIDL!{#[uuid(0x77a416e7, 0x2ac6, 0x3d0e, 0x98, 0xff, 0x3b, 0xa0, 0xf5, 0x86, 0xf5, 0x6f)]
interface _RSAOAEPKeyExchangeFormatter(_RSAOAEPKeyExchangeFormatterVtbl): IDispatch(IDispatchVtbl)  
{}}