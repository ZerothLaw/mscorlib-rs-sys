// customattribute.rs - MIT License
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

use winapi::um::unknwnbase::{IUnknown};

use system::reflection::{_MemberInfo,_Type};

//struct __declspec(uuid("9dc6ac40-edfa-3e34-9ad1-b7a0a9e3a40a"))
STRUCT!{struct CustomAttributeTypedArgument
{
    m_value: *mut IUnknown,
    m_argumentType: *mut _Type,
}}

//struct __declspec(uuid("7fc47a26-aa2e-32ea-bde4-01a490842d87"))
STRUCT!{struct CustomAttributeNamedArgument
{
    m_memberInfo: *mut _MemberInfo,
    m_value: CustomAttributeTypedArgument,
}}