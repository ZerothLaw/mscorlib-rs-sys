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

use winapi::ctypes::c_char;
//struct __declspec(uuid("ca2bcdb4-3a7e-33e8-80ed-d32475adef33"))
STRUCT!{struct SByte {
    m_value: c_char, 
}}
// IComparable, IFormattable, IConvertible
//     , IComparable<SByte>, IEquatable<SByte>