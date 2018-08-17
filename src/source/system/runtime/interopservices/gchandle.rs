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

use system::intptr::IntPtr;

//enum __declspec(uuid("0e71f38e-c5e1-3094-9487-5c7dd1e998ec"))
ENUM!{enum GCHandleType
{
    GCHandleType_Weak = 0,
    GCHandleType_WeakTrackResurrection = 1,
    GCHandleType_Normal = 2,
    GCHandleType_Pinned = 3,
}}

//struct __declspec(uuid("66e1f723-e57f-35ce-8306-3c09fb68a322"))
STRUCT!{struct GCHandle
{
    m_handle: IntPtr,
}} //implementation can probably be done here?