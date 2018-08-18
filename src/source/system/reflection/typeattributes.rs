// typeattributes.rs - MIT License
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

//enum __declspec(uuid("28ee6224-fd72-3bdf-b248-ba9102fceb14"))
ENUM!{enum TypeAttributes {
    TypeAttributes_VisibilityMask = 7,
    TypeAttributes_NotPublic = 0,
    TypeAttributes_Public = 1,
    TypeAttributes_NestedPublic = 2,
    TypeAttributes_NestedPrivate = 3,
    TypeAttributes_NestedFamily = 4,
    TypeAttributes_NestedAssembly = 5,
    TypeAttributes_NestedFamANDAssem = 6,
    TypeAttributes_NestedFamORAssem = 7,
    TypeAttributes_LayoutMask = 24,
    TypeAttributes_AutoLayout = 0,
    TypeAttributes_SequentialLayout = 8,
    TypeAttributes_ExplicitLayout = 16,
    TypeAttributes_ClassSemanticsMask = 32,
    TypeAttributes_Class = 0,
    TypeAttributes_Interface = 32,
    TypeAttributes_Abstract = 128,
    TypeAttributes_Sealed = 256,
    TypeAttributes_SpecialName = 1024,
    TypeAttributes_Import = 4096,
    TypeAttributes_Serializable = 8192,
    TypeAttributes_StringFormatMask = 196608,
    TypeAttributes_AnsiClass = 0,
    TypeAttributes_UnicodeClass = 65536,
    TypeAttributes_AutoClass = 131072,
    TypeAttributes_CustomFormatClass = 196608,
    TypeAttributes_CustomFormatMask = 12582912,
    TypeAttributes_BeforeFieldInit = 1048576,
    TypeAttributes_ReservedMask = 264192,
    TypeAttributes_RTSpecialName = 2048,
    TypeAttributes_HasSecurity = 262144,
}}
