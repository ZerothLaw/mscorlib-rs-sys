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
RIDL!{#[uuid(0x8c248251, 0x3e6c, 0x3151, 0x9f, 0x8e, 0xa2, 0x55, 0xfb, 0x8d, 0x2b, 0x12)]
interface _TextElementEnumerator(_TextElementEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}}