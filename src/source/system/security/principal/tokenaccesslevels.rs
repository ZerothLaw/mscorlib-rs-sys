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

//enum __declspec(uuid("10a8b906-2f7a-327c-87ab-1a95a9b5e23e"))
ENUM!{enum TokenAccessLevels
{
    TokenAccessLevels_AssignPrimary = 1,
    TokenAccessLevels_Duplicate = 2,
    TokenAccessLevels_Impersonate = 4,
    TokenAccessLevels_Query = 8,
    TokenAccessLevels_QuerySource = 16,
    TokenAccessLevels_AdjustPrivileges = 32,
    TokenAccessLevels_AdjustGroups = 64,
    TokenAccessLevels_AdjustDefault = 128,
    TokenAccessLevels_AdjustSessionId = 256,
    TokenAccessLevels_Read = 131080,
    TokenAccessLevels_Write = 131296,
    TokenAccessLevels_AllAccess = 983551,
    TokenAccessLevels_MaximumAllowed = 33554432,
}}