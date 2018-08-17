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

//enum __declspec(uuid("f62ff05f-99ce-30db-8344-2b2c26f5765c"))
ENUM!{enum DateTimeStyles
{
    DateTimeStyles_None = 0,
    DateTimeStyles_AllowLeadingWhite = 1,
    DateTimeStyles_AllowTrailingWhite = 2,
    DateTimeStyles_AllowInnerWhite = 4,
    DateTimeStyles_AllowWhiteSpaces = 7,
    DateTimeStyles_NoCurrentDateDefault = 8,
    DateTimeStyles_AdjustToUniversal = 16,
    DateTimeStyles_AssumeLocal = 32,
    DateTimeStyles_AssumeUniversal = 64,
    DateTimeStyles_RoundTripKind = 128,
}}