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

use winapi::um::oaidl::SAFEARRAY;
//struct __declspec(uuid("11d31042-14c0-3b5c-87d0-a2cd40803cb5"))
STRUCT!{struct ParameterModifier {
    byRef: *mut SAFEARRAY, //bool[]
}}
//array of bools to indicate if param is by reference or not
