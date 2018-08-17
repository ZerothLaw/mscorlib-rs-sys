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

use winapi::um::unknwnbase::{IUnknown};

//struct __declspec(uuid("f8fc5d7c-8215-3e65-befb-11e8172606fe"))
STRUCT!{struct RuntimeMethodHandle {
    m_value: *mut IUnknown,
}}

//struct __declspec(uuid("27b33bd9-e6f7-3148-911d-f67340a5353f"))
STRUCT!{struct RuntimeFieldHandle {
    m_ptr: *mut IUnknown,
}}

//struct __declspec(uuid("8531f85a-746b-3db5-a45f-9bac4bd02d8b"))
STRUCT!{struct ModuleHandle {
    m_ptr: *mut IUnknown,
}}