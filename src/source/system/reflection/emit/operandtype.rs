// operandtype.rs - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

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
