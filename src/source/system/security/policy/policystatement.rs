// policystatement.rs - MIT License
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

//enum __declspec(uuid("338d2529-b3d6-37f1-bb01-404698dc537b"))
ENUM!{enum PolicyStatementAttribute
{
    PolicyStatementAttribute_Nothing = 0,
    PolicyStatementAttribute_Exclusive = 1,
    PolicyStatementAttribute_LevelFinal = 2,
    PolicyStatementAttribute_All = 3,
}}

RIDL!{#[uuid(0x3eefd1fc, 0x4d8d, 0x3177, 0x99, 0xf6, 0x6c, 0x19, 0xd9, 0xe0, 0x88, 0xd3)]
interface _PolicyStatement(_PolicyStatementVtbl): IDispatch(IDispatchVtbl)  
{}} // ISecurityPolicyEncodable, ISecurityEncodable