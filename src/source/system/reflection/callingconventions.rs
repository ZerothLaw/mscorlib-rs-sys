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

//enum __declspec(uuid("fd67ebe2-30de-3fbe-896b-81da2e455137"))
ENUM!{enum CallingConventions {
    CallingConventions_Standard = 1,
    CallingConventions_VarArgs = 2,
    CallingConventions_Any = 3,
    CallingConventions_HasThis = 32,
    CallingConventions_ExplicitThis = 64,
}}