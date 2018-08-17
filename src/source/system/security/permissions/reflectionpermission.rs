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

//enum __declspec(uuid("44c2f476-9e95-3d5a-b666-fdbef071494e"))
ENUM!{enum ReflectionPermissionFlag
{
    ReflectionPermissionFlag_NoFlags = 0,
    ReflectionPermissionFlag_TypeInformation = 1,
    ReflectionPermissionFlag_MemberAccess = 2,
    ReflectionPermissionFlag_ReflectionEmit = 4,
    ReflectionPermissionFlag_AllFlags = 7,
}}

RIDL!{#[uuid(0xaeb3727f, 0x5c3a, 0x34c4, 0xbf, 0x18, 0xa3, 0x8f, 0x08, 0x8a, 0xc8, 0xc7)]
interface _ReflectionPermission(_ReflectionPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission, IBuiltInPermission