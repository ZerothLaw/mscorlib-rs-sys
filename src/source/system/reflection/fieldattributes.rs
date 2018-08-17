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

//enum __declspec(uuid("c8679e0a-1c67-3a20-8645-0d930f529031"))
ENUM!{enum FieldAttributes {
    FieldAttributes_FieldAccessMask = 7,
    FieldAttributes_PrivateScope = 0,
    FieldAttributes_Private = 1,
    FieldAttributes_FamANDAssem = 2,
    FieldAttributes_Assembly = 3,
    FieldAttributes_Family = 4,
    FieldAttributes_FamORAssem = 5,
    FieldAttributes_Public = 6,
    FieldAttributes_Static = 16,
    FieldAttributes_InitOnly = 32,
    FieldAttributes_Literal = 64,
    FieldAttributes_NotSerialized = 128,
    FieldAttributes_SpecialName = 512,
    FieldAttributes_PinvokeImpl = 8192,
    FieldAttributes_ReservedMask = 38144,
    FieldAttributes_RTSpecialName = 1024,
    FieldAttributes_HasFieldMarshal = 4096,
    FieldAttributes_HasDefault = 32768,
    FieldAttributes_HasFieldRVA = 256,
}}