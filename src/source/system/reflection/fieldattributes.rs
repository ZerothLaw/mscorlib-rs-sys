// fieldattributes.rs - MIT License
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

//enum __declspec(uuid("c8679e0a-1c67-3a20-8645-0d930f529031"))
ENUM!{enum FieldAttributes {
    FieldAttributes_FieldAccessMask = 7,
    FieldAttributes_PrivateScope = 0,
    FieldAttributes_Private = 1,
    FieldAttributes_FamANDAssem = 2,
    FieldAttributes_Assembly = 3,
    FieldAttributes_Family = 4,
    FieldAttributes_FamORAssem = 5,
    FieldAttributes_Public = 6,
    FieldAttributes_Static = 16,
    FieldAttributes_InitOnly = 32,
    FieldAttributes_Literal = 64,
    FieldAttributes_NotSerialized = 128,
    FieldAttributes_SpecialName = 512,
    FieldAttributes_PinvokeImpl = 8192,
    FieldAttributes_ReservedMask = 38144,
    FieldAttributes_RTSpecialName = 1024,
    FieldAttributes_HasFieldMarshal = 4096,
    FieldAttributes_HasDefault = 32768,
    FieldAttributes_HasFieldRVA = 256,
}}