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

//enum __declspec(uuid("0df04a9b-dddc-3777-a6b1-9604b5ced191"))
ENUM!{enum FileDialogPermissionAccess
{
    FileDialogPermissionAccess_None = 0,
    FileDialogPermissionAccess_Open = 1,
    FileDialogPermissionAccess_Save = 2,
    FileDialogPermissionAccess_OpenSave = 3,
}}

RIDL!{#[uuid(0xa8b7138c, 0x8932, 0x3d78, 0xa5, 0x85, 0xa9, 0x15, 0x69, 0xc7, 0x43, 0xac)]
interface _FileDialogPermission(_FileDialogPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}