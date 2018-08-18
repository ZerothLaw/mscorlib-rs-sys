// parameterattributes.rs - MIT License
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

//enum __declspec(uuid("688a6ff0-5727-32d2-8228-6e838a822616"))
ENUM!{enum ParameterAttributes {
    ParameterAttributes_None = 0,
    ParameterAttributes_In = 1,
    ParameterAttributes_Out = 2,
    ParameterAttributes_Lcid = 4,
    ParameterAttributes_Retval = 8,
    ParameterAttributes_Optional = 16,
    ParameterAttributes_ReservedMask = 61440,
    ParameterAttributes_HasDefault = 4096,
    ParameterAttributes_HasFieldMarshal = 8192,
    ParameterAttributes_Reserved3 = 16384,
    ParameterAttributes_Reserved4 = 32768,
}}