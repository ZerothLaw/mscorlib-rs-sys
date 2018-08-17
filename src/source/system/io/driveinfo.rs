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

//enum __declspec(uuid("72e8197d-904b-3371-ae0e-b70d9d53771c"))
ENUM!{enum DriveType
{
    DriveType_Unknown = 0,
    DriveType_NoRootDirectory = 1,
    DriveType_Removable = 2,
    DriveType_Fixed = 3,
    DriveType_Network = 4,
    DriveType_CDRom = 5,
    DriveType_Ram = 6,
}}

RIDL!{#[uuid(0xce83a763, 0x940f, 0x341f, 0xb8, 0x80, 0x33, 0x23, 0x25, 0xeb, 0x6f, 0x4b)]
interface _DriveInfo(_DriveInfoVtbl): IDispatch(IDispatchVtbl)  
{}}