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

RIDL!{#[uuid(0x54b0afb1, 0xe7d3, 0x3770, 0xbb, 0x0e, 0x75, 0xa9, 0x5e, 0x8d, 0x26, 0x56)]
interface _FirstMatchCodeGroup(_FirstMatchCodeGroupVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeGroup