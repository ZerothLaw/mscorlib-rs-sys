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

use system::reflection::cominterfaces::_MemberInfo;
use system::reflection::cominterfaces::_Type;

//struct __declspec(uuid("9dc6ac40-edfa-3e34-9ad1-b7a0a9e3a40a"))
STRUCT!{struct CustomAttributeTypedArgument
{
    m_value: *mut IUnknown,
    m_argumentType: *mut _Type,
}}

//struct __declspec(uuid("7fc47a26-aa2e-32ea-bde4-01a490842d87"))
STRUCT!{struct CustomAttributeNamedArgument
{
    m_memberInfo: *mut _MemberInfo,
    m_value: CustomAttributeTypedArgument,
}}