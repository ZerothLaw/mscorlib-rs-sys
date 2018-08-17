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

RIDL!{#[uuid(0x2b67cece, 0x71c3, 0x36a9, 0xa1, 0x36, 0x92, 0x5c, 0xcc, 0x19, 0x35, 0xa8)]
interface _Array(_ArrayVtbl): IDispatch(IDispatchVtbl)  
{}} //ICloneable, IList, IStructuralComparable, IStructuralEquatable 