// commonenums.rs - MIT License
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

//enum __declspec(uuid("72b06367-de53-3111-9c49-b816efee3148"))
ENUM!{enum FormatterTypeStyle
{
    FormatterTypeStyle_TypesWhenNeeded = 0,
    FormatterTypeStyle_TypesAlways = 1,
    FormatterTypeStyle_XsdString = 2,
}}

//enum __declspec(uuid("f18130e7-bd6c-37f4-9488-35f9fb832ac7"))
ENUM!{enum FormatterAssemblyStyle
{
    FormatterAssemblyStyle_Simple = 0,
    FormatterAssemblyStyle_Full = 1,
}}

//enum __declspec(uuid("c5d299ac-63b0-3448-bcb7-6aa9b5eb598e"))
ENUM!{enum TypeFilterLevel
{
    TypeFilterLevel_Low = 2,
    TypeFilterLevel_Full = 3,
}}