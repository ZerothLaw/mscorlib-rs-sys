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

//enum __declspec(uuid("03c85cd9-d760-3aa8-94bd-f774407391cb"))
ENUM!{enum EventAttributes {
    EventAttributes_None = 0,
    EventAttributes_SpecialName = 512,
    EventAttributes_ReservedMask = 1024,
    EventAttributes_RTSpecialName = 1024,
}}