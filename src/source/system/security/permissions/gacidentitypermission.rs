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

RIDL!{#[uuid(0x5f19e082, 0x26f8, 0x3361, 0xb3, 0x38, 0x9b, 0xac, 0xb9, 0x88, 0x09, 0xa4)]
interface _GacIdentityPermissionAttribute(_GacIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0xa9637792, 0x5be8, 0x3c93, 0xa5, 0x01, 0x49, 0xf0, 0xe8, 0x40, 0xde, 0x38)]
interface _GacIdentityPermission(_GacIdentityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IBuiltInPermission
