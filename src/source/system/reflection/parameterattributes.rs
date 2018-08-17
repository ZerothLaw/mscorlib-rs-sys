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

//enum __declspec(uuid("688a6ff0-5727-32d2-8228-6e838a822616"))
ENUM!{enum ParameterAttributes {
    ParameterAttributes_None = 0,
    ParameterAttributes_In = 1,
    ParameterAttributes_Out = 2,
    ParameterAttributes_Lcid = 4,
    ParameterAttributes_Retval = 8,
    ParameterAttributes_Optional = 16,
    ParameterAttributes_ReservedMask = 61440,
    ParameterAttributes_HasDefault = 4096,
    ParameterAttributes_HasFieldMarshal = 8192,
    ParameterAttributes_Reserved3 = 16384,
    ParameterAttributes_Reserved4 = 32768,
}}