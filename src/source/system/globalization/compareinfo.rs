// compareinfo.rs - MIT License
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

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("fdbf0369-d278-3320-b9ce-0e0719380c0f"))
ENUM!{enum CompareOptions
{
    CompareOptions_None = 0,
    CompareOptions_IgnoreCase = 1,
    CompareOptions_IgnoreNonSpace = 2,
    CompareOptions_IgnoreSymbols = 4,
    CompareOptions_IgnoreKanaType = 8,
    CompareOptions_IgnoreWidth = 16,
    CompareOptions_OrdinalIgnoreCase = 268435456,
    CompareOptions_StringSort = 536870912,
    CompareOptions_Ordinal = 1073741824,
}}

RIDL!{#[uuid(0x505defe5, 0xaefa, 0x3e23, 0x82, 0xb0, 0xd5, 0xeb, 0x08, 0x5b, 0xb8, 0x40)]
interface _CompareInfo(_CompareInfoVtbl): IDispatch(IDispatchVtbl)  
{}}
