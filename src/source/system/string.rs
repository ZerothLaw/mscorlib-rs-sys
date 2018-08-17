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
RIDL!{#[uuid(0x36936699, 0xfc79, 0x324d, 0xab, 0x43, 0xe3, 0x3c, 0x1f, 0x94, 0xe2, 0x63)]
interface _String(_StringVtbl): IDispatch(IDispatchVtbl)  
{}}
//IComparable, ICloneable, IConvertible, IEnumerable, IComparable<String>, IEnumerable<char>, IEquatable<String>