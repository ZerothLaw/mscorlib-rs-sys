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

//enum __declspec(uuid("dfaecf33-4728-382d-a34d-c1b0392f8b73"))
ENUM!{enum PermissionState
{
    PermissionState_Unrestricted = 1,
    PermissionState_None = 0,
}}

RIDL!{#[uuid(0x7c6b06d1, 0x63ad, 0x35ef, 0xa9, 0x38, 0x14, 0x9b, 0x4a, 0xd9, 0xa7, 0x1f)]
interface _PrincipalPermission(_PrincipalPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} // IPermission, IUnrestrictedPermission, ISecurityEncodable, IBuiltInPermission