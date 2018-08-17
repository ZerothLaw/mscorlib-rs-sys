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

//enum __declspec(uuid("0d6e31df-3a76-3054-a8eb-150e92300f89"))
ENUM!{enum IsolatedStorageContainment
{
    IsolatedStorageContainment_None = 0,
    IsolatedStorageContainment_DomainIsolationByUser = 16,
    IsolatedStorageContainment_ApplicationIsolationByUser = 21,
    IsolatedStorageContainment_AssemblyIsolationByUser = 32,
    IsolatedStorageContainment_DomainIsolationByMachine = 48,
    IsolatedStorageContainment_AssemblyIsolationByMachine = 64,
    IsolatedStorageContainment_ApplicationIsolationByMachine = 69,
    IsolatedStorageContainment_DomainIsolationByRoamingUser = 80,
    IsolatedStorageContainment_AssemblyIsolationByRoamingUser = 96,
    IsolatedStorageContainment_ApplicationIsolationByRoamingUser = 101,
    IsolatedStorageContainment_AdministerIsolatedStorageByUser = 112,
    IsolatedStorageContainment_UnrestrictedIsolatedStorage = 240,
}}

RIDL!{#[uuid(0x7fee7903, 0xf97c, 0x3350, 0xad, 0x42, 0x19, 0x6b, 0x00, 0xad, 0x25, 0x64)]
interface _IsolatedStoragePermission(_IsolatedStoragePermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission