// fileiopermission.rs - MIT License
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

//enum __declspec(uuid("ca10c1a1-9fdc-36a3-ad74-8fac60e6541c"))
ENUM!{enum FileIOPermissionAccess
{
    FileIOPermissionAccess_NoAccess = 0,
    FileIOPermissionAccess_Read = 1,
    FileIOPermissionAccess_Write = 2,
    FileIOPermissionAccess_Append = 4,
    FileIOPermissionAccess_PathDiscovery = 8,
    FileIOPermissionAccess_AllAccess = 15,
}}

RIDL!{#[uuid(0xa2ed7efc, 0x8e59, 0x3ccc, 0xae, 0x92, 0xea, 0x23, 0x77, 0xf4, 0xd5, 0xef)]
interface _FileIOPermission(_FileIOPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}