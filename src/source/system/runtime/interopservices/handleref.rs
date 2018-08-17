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
use system::intptr::IntPtr;
//struct __declspec(uuid("c71dce2b-b87f-37a9-89ed-f1145955bcd6"))
STRUCT!{struct HandleRef
{
    m_wrapper: *mut IUnknown,
    m_handle: IntPtr,
}}

