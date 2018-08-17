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

RIDL!{#[uuid(0xba3e053f, 0xade3, 0x3233, 0x87, 0x4a, 0x16, 0xe6, 0x24, 0xc9, 0xa4, 0x9b)]
interface _NamedPermissionSet(_NamedPermissionSetVtbl): IDispatch(IDispatchVtbl)  
{}}//PermissionSet