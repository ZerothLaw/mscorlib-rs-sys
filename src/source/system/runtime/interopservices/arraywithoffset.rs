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

use winapi::ctypes::c_long;
use winapi::um::unknwnbase::{IUnknown};
//struct __declspec(uuid("8351108f-34e3-3cc9-bf5a-c76c48060835"))
STRUCT!{struct ArrayWithOffset {
    m_array: *mut IUnknown,
    m_offset: c_long, 
    m_count: c_long,
}}