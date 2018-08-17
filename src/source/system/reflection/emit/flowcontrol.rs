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

//enum __declspec(uuid("75a7861c-767e-3a5e-a57b-6ec136009654"))
ENUM!{enum FlowControl
{
    FlowControl_Branch = 0,
    FlowControl_Break = 1,
    FlowControl_Call = 2,
    FlowControl_Cond_Branch = 3,
    FlowControl_Meta = 4,
    FlowControl_Next = 5,
    FlowControl_Phi = 6,
    FlowControl_Return = 7,
    FlowControl_Throw = 8,
}}