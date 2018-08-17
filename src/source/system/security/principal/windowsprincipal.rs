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

//enum __declspec(uuid("8b7e18b8-3e96-3a4c-82cb-3d13fa15a32f"))
ENUM!{enum WindowsBuiltInRole
{
    WindowsBuiltInRole_Administrator = 544,
    WindowsBuiltInRole_User = 545,
    WindowsBuiltInRole_Guest = 546,
    WindowsBuiltInRole_PowerUser = 547,
    WindowsBuiltInRole_AccountOperator = 548,
    WindowsBuiltInRole_SystemOperator = 549,
    WindowsBuiltInRole_PrintOperator = 550,
    WindowsBuiltInRole_BackupOperator = 551,
    WindowsBuiltInRole_Replicator = 552,
}}

RIDL!{#[uuid(0x6c42baf9, 0x1893, 0x34fc, 0xb3, 0xaf, 0x06, 0x93, 0x1e, 0x9b, 0x34, 0xa3)]
interface _WindowsPrincipal(_WindowsPrincipalVtbl): IDispatch(IDispatchVtbl)  
{}} //ClaimsPrincipal