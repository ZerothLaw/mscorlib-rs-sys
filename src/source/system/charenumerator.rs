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

use winapi::ctypes::{c_char, c_long};
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

STRUCT!{struct CharEnumerator{
    str_: BSTR,
    index: c_long, 
    currentElement: c_char, 
}}
//implements IEnumerator, ICloneable, IEnumerator<char>, IDisposable 

RIDL!{#[uuid(0x1dd627fc, 0x89e3, 0x384f, 0xbb, 0x9d, 0x58, 0xcb, 0x4e, 0xfb, 0x94, 0x56)]
interface _CharEnumerator(_CharEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}}