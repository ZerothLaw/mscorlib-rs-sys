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

ENUM!{enum MemberTypes {
    MemberTypes_Constructor = 1,
    MemberTypes_Event = 2,
    MemberTypes_Field = 4,
    MemberTypes_Method = 8,
    MemberTypes_Property = 16,
    MemberTypes_TypeInfo = 32,
    MemberTypes_Custom = 64,
    MemberTypes_NestedType = 128,
    MemberTypes_All = 191,
}}