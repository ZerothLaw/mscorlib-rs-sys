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

//enum __declspec(uuid("b30fd15e-ced6-3977-8151-0d50e79cd703"))
ENUM!{enum UIPermissionWindow
{
    UIPermissionWindow_NoWindows = 0,
    UIPermissionWindow_SafeSubWindows = 1,
    UIPermissionWindow_SafeTopLevelWindows = 2,
    UIPermissionWindow_AllWindows = 3,
}}

//enum __declspec(uuid("9e5c3c99-d046-3fe5-9921-21cf0f0a08ff"))
ENUM!{enum UIPermissionClipboard
{
    UIPermissionClipboard_NoClipboard = 0,
    UIPermissionClipboard_OwnClipboard = 1,
    UIPermissionClipboard_AllClipboard = 2,
}}

RIDL!{#[uuid(0x47698389, 0xf182, 0x3a67, 0x87, 0xdf, 0xae, 0xd4, 0x90, 0xe1, 0x4d, 0xc6)]
interface _UIPermission(_UIPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission, IBuiltInPermission