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

//enum __declspec(uuid("8abd8cb3-a365-32f9-9914-f08ec1fec4ca"))
ENUM!{enum OpCodeType
{
    OpCodeType_Annotation = 0,
    OpCodeType_Macro = 1,
    OpCodeType_Nternal = 2,
    OpCodeType_Objmodel = 3,
    OpCodeType_Prefix = 4,
    OpCodeType_Primitive = 5,
}}