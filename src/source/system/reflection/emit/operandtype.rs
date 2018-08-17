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

//enum __declspec(uuid("b125618b-1b4e-37c3-b31a-331d6021b52d"))
ENUM!{enum OperandType
{
    OperandType_InlineBrTarget = 0,
    OperandType_InlineField = 1,
    OperandType_InlineI = 2,
    OperandType_InlineI8 = 3,
    OperandType_InlineMethod = 4,
    OperandType_InlineNone = 5,
    OperandType_InlinePhi = 6,
    OperandType_InlineR = 7,
    OperandType_InlineSig = 9,
    OperandType_InlineString = 10,
    OperandType_InlineSwitch = 11,
    OperandType_InlineTok = 12,
    OperandType_InlineType = 13,
    OperandType_InlineVar = 14,
    OperandType_ShortInlineBrTarget = 15,
    OperandType_ShortInlineI = 16,
    OperandType_ShortInlineR = 17,
    OperandType_ShortInlineVar = 18,
}}
