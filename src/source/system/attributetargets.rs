// attributetargets.rs - MIT License
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

//enum __declspec(uuid("9bc2306f-4971-38f5-b861-f19c022274a0"))
ENUM!{enum AttributeTargets
{
    AttributeTargets_Assembly = 1,
    AttributeTargets_Module = 2,
    AttributeTargets_Class = 4,
    AttributeTargets_Struct = 8,
    AttributeTargets_Enum = 16,
    AttributeTargets_Constructor = 32,
    AttributeTargets_Method = 64,
    AttributeTargets_Property = 128,
    AttributeTargets_Field = 256,
    AttributeTargets_Event = 512,
    AttributeTargets_Interface = 1024,
    AttributeTargets_Parameter = 2048,
    AttributeTargets_Delegate = 4096,
    AttributeTargets_ReturnValue = 8192,
    AttributeTargets_GenericParameter = 16384,
    AttributeTargets_All = 32767,
}}