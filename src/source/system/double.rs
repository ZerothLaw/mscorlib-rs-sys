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

use winapi::ctypes::c_double;

//struct __declspec(uuid("0f4f147f-4369-3388-8e4b-71e20c96f9ad"))
STRUCT!{struct Double {
    m_value: c_double,
}}
//IComparable, IFormattable, IConvertible
//        , IComparable<Double>, IEquatable<Double>