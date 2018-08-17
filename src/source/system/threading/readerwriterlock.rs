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

RIDL!{#[uuid(0xad89b568, 0x4fd4, 0x3f8d, 0x83, 0x27, 0xb3, 0x96, 0xb2, 0x0a, 0x46, 0x0e)]
interface _ReaderWriterLock(_ReaderWriterLockVtbl): IDispatch(IDispatchVtbl)  
{}} //CriticalFinalizerObject
