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

//enum __declspec(uuid("68da8301-be1b-3c22-b9f2-db8f48694ddd"))
ENUM!{enum PortableExecutableKinds {
    PortableExecutableKinds_NotAPortableExecutableImage = 0,
    PortableExecutableKinds_ILOnly = 1,
    PortableExecutableKinds_Required32Bit = 2,
    PortableExecutableKinds_PE32Plus = 4,
    PortableExecutableKinds_Unmanaged32Bit = 8,
}}

//enum __declspec(uuid("51191552-c65e-360d-ba21-9f0e454fd59f"))
ENUM!{enum ImageFileMachine {
    ImageFileMachine_I386 = 332,
    ImageFileMachine_IA64 = 512,
    ImageFileMachine_AMD64 = 34404,
    ImageFileMachine_ARM = 452, 
}}

//Module implements by _Module, ISerializable, ICustomAttributeProvider