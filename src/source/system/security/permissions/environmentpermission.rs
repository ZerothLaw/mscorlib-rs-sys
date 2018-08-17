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

//enum __declspec(uuid("ec73fceb-1aea-3a57-b953-21368e992507"))
ENUM!{enum EnvironmentPermissionAccess
{
    EnvironmentPermissionAccess_NoAccess = 0,
    EnvironmentPermissionAccess_Read = 1,
    EnvironmentPermissionAccess_Write = 2,
    EnvironmentPermissionAccess_AllAccess = 3,
}}

RIDL!{#[uuid(0x0720590d, 0x5218, 0x352a, 0xa3, 0x37, 0x54, 0x49, 0xe6, 0xbd, 0x19, 0xda)]
interface _EnvironmentPermission(_EnvironmentPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission, IBuiltInPermission
