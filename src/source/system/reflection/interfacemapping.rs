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

use system::reflection::_Type;

//struct __declspec(uuid("5f7a2664-4778-3d72-a78f-d38b6b00180d"))
STRUCT!{struct InterfaceMapping
{
    TargetType: *mut _Type,
    interfaceType: *mut _Type,
    TargetMethods: *mut SAFEARRAY, //MethodInfo[]
    InterfaceMethods: *mut SAFEARRAY, //MethodInfo[]
}}