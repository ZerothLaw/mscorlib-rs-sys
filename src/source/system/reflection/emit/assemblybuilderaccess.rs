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

//enum __declspec(uuid("f0778630-ac34-3d71-9fab-617f61243065"))
ENUM!{enum AssemblyBuilderAccess
{
    AssemblyBuilderAccess_Run = 1,
    AssemblyBuilderAccess_Save = 2,
    AssemblyBuilderAccess_RunAndSave = 3,
    AssemblyBuilderAccess_ReflectionOnly = 6,
    AssemblyBuilderAccess_RunAndCollect = 9,
}}