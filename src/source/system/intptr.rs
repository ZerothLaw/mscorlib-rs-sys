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

use std::ops::Deref;

use winapi::ctypes::c_void;

//implements ISerializable
STRUCT!{struct IntPtr{
    m_value: *mut c_void,
}}

impl Deref for IntPtr {
    type Target = c_void;
    #[inline]
    fn deref(&self) -> &c_void {
        unsafe {&*self.m_value}
    }
}