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

RIDL!{#[uuid(0x87f55344, 0x17e0, 0x30fd, 0x8e, 0xb9, 0x38, 0xea, 0xf6, 0xa1, 0x9b, 0x3f)]
interface _SynchronizationLockException(_SynchronizationLockExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}//SystemException