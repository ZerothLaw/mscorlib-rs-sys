// applicationtrust.rs - MIT License
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

//enum __declspec(uuid("d93eaca8-8176-387b-9667-6d32b504047b"))
ENUM!{enum ApplicationVersionMatch {
    ApplicationVersionMatch_MatchExactVersion = 0,
    ApplicationVersionMatch_MatchAllVersions = 1,
}}

RIDL!{#[uuid(0xe66a9755, 0x58e2, 0x3fcb, 0xa2, 0x65, 0x83, 0x58, 0x51, 0xcb, 0xf0, 0x63)]
interface _ApplicationTrust(_ApplicationTrustVtbl): IDispatch(IDispatchVtbl)  
{}} //EvidenceBase, ISecurityEncodable