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

use winapi::ctypes::c_short;
//struct __declspec(uuid("206daf34-5ba5-3504-8a19-d57699561886"))
STRUCT!{struct Int16 {
    m_value: c_short,
}}
// IComparable, IFormattable, IConvertible
// , IComparable<Int16>, IEquatable<Int16>