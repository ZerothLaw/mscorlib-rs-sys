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

ENUM!{enum MethodAttributes {
    MethodAttributes_MemberAccessMask = 7,
    MethodAttributes_PrivateScope = 0,
    MethodAttributes_Private = 1,
    MethodAttributes_FamANDAssem = 2,
    MethodAttributes_Assembly = 3,
    MethodAttributes_Family = 4,
    MethodAttributes_FamORAssem = 5,
    MethodAttributes_Public = 6,
    MethodAttributes_Static = 16,
    MethodAttributes_Final = 32,
    MethodAttributes_Virtual = 64,
    MethodAttributes_HideBySig = 128,
    MethodAttributes_CheckAccessOnOverride = 512,
    MethodAttributes_VtableLayoutMask = 256,
    MethodAttributes_ReuseSlot = 0,
    MethodAttributes_NewSlot = 256,
    MethodAttributes_Abstract = 1024,
    MethodAttributes_SpecialName = 2048,
    MethodAttributes_PinvokeImpl = 8192,
    MethodAttributes_UnmanagedExport = 8,
    MethodAttributes_RTSpecialName = 4096,
    MethodAttributes_ReservedMask = 53248,
    MethodAttributes_HasSecurity = 16384,
    MethodAttributes_RequireSecObject = 32768,
}}