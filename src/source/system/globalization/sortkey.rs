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
RIDL!{#[uuid(0xf4c70e15, 0x2ca6, 0x3e90, 0x96, 0xed, 0x92, 0xe2, 0x84, 0x91, 0xf5, 0x38)]
interface _SortKey(_SortKeyVtbl): IDispatch(IDispatchVtbl)  
{}}
