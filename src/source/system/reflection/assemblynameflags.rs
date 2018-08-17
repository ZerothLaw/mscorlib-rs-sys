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


//enum __declspec(uuid("981dc77e-ce21-3753-92da-3c4a0cc7aa44"))
ENUM!{enum AssemblyNameFlags {
    AssemblyNameFlags_None = 0,
    AssemblyNameFlags_PublicKey = 1,
    AssemblyNameFlags_EnableJITcompileOptimizer = 16384,
    AssemblyNameFlags_EnableJITcompileTracking = 32768,
    AssemblyNameFlags_Retargetable = 256,
}}

//enum __declspec(uuid("56b1cccb-6490-396d-8c09-2257259f3caa"))
ENUM!{enum ProcessorArchitecture {
    ProcessorArchitecture_None = 0,
    ProcessorArchitecture_MSIL = 1,
    ProcessorArchitecture_X86 = 2,
    ProcessorArchitecture_IA64 = 3,
    ProcessorArchitecture_Amd64 = 4,
    ProcessorArchitecture_Arm = 5,
}}