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

//enum __declspec(uuid("816c979c-d3d2-3101-b5ca-e4a5c5e966fa"))
ENUM!{enum PropertyAttributes {
    PropertyAttributes_None = 0,
    PropertyAttributes_SpecialName = 512,
    PropertyAttributes_ReservedMask = 62464,
    PropertyAttributes_RTSpecialName = 1024,
    PropertyAttributes_HasDefault = 4096,
    PropertyAttributes_Reserved2 = 8192,
    PropertyAttributes_Reserved3 = 16384,
    PropertyAttributes_Reserved4 = 32768,
}}