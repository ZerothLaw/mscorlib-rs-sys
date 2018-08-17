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

//enum __declspec(uuid("a2c06560-e728-39d5-8230-7eb08001c79e"))
ENUM!{enum LeaseState
{
    LeaseState_Null = 0,
    LeaseState_Initial = 1,
    LeaseState_Active = 2,
    LeaseState_Renewing = 3,
    LeaseState_Expired = 4,
}}
