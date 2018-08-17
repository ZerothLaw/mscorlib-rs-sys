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

RIDL!{#[uuid(0x6b3f9834, 0x1725, 0x38c5, 0x95, 0x5e, 0x20, 0xf0, 0x51, 0xd0, 0x67, 0xbd)]
interface _CriticalFinalizerObject(_CriticalFinalizerObjectVtbl): IDispatch(IDispatchVtbl)  
{}}