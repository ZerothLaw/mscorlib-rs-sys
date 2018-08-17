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

//enum __declspec(uuid("ca10c1a1-9fdc-36a3-ad74-8fac60e6541c"))
ENUM!{enum FileIOPermissionAccess
{
    FileIOPermissionAccess_NoAccess = 0,
    FileIOPermissionAccess_Read = 1,
    FileIOPermissionAccess_Write = 2,
    FileIOPermissionAccess_Append = 4,
    FileIOPermissionAccess_PathDiscovery = 8,
    FileIOPermissionAccess_AllAccess = 15,
}}

RIDL!{#[uuid(0xa2ed7efc, 0x8e59, 0x3ccc, 0xae, 0x92, 0xea, 0x23, 0x77, 0xf4, 0xd5, 0xef)]
interface _FileIOPermission(_FileIOPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}