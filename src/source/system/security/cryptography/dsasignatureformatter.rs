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

RIDL!{#[uuid(0x4b5fc561, 0x5983, 0x31e4, 0x90, 0x3b, 0x14, 0x04, 0x23, 0x1b, 0x2c, 0x89)]
interface _DSASignatureFormatter(_DSASignatureFormatterVtbl): IDispatch(IDispatchVtbl)  
{}} //AsymmetricSignatureFormatter