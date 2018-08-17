// windowsprincipal.rs - MIT License
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

//enum __declspec(uuid("8b7e18b8-3e96-3a4c-82cb-3d13fa15a32f"))
ENUM!{enum WindowsBuiltInRole
{
    WindowsBuiltInRole_Administrator = 544,
    WindowsBuiltInRole_User = 545,
    WindowsBuiltInRole_Guest = 546,
    WindowsBuiltInRole_PowerUser = 547,
    WindowsBuiltInRole_AccountOperator = 548,
    WindowsBuiltInRole_SystemOperator = 549,
    WindowsBuiltInRole_PrintOperator = 550,
    WindowsBuiltInRole_BackupOperator = 551,
    WindowsBuiltInRole_Replicator = 552,
}}

RIDL!{#[uuid(0x6c42baf9, 0x1893, 0x34fc, 0xb3, 0xaf, 0x06, 0x93, 0x1e, 0x9b, 0x34, 0xa3)]
interface _WindowsPrincipal(_WindowsPrincipalVtbl): IDispatch(IDispatchVtbl)  
{}} //ClaimsPrincipal