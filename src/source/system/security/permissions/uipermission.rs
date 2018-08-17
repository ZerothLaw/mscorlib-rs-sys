// uipermission.rs - MIT License
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

//enum __declspec(uuid("b30fd15e-ced6-3977-8151-0d50e79cd703"))
ENUM!{enum UIPermissionWindow
{
    UIPermissionWindow_NoWindows = 0,
    UIPermissionWindow_SafeSubWindows = 1,
    UIPermissionWindow_SafeTopLevelWindows = 2,
    UIPermissionWindow_AllWindows = 3,
}}

//enum __declspec(uuid("9e5c3c99-d046-3fe5-9921-21cf0f0a08ff"))
ENUM!{enum UIPermissionClipboard
{
    UIPermissionClipboard_NoClipboard = 0,
    UIPermissionClipboard_OwnClipboard = 1,
    UIPermissionClipboard_AllClipboard = 2,
}}

RIDL!{#[uuid(0x47698389, 0xf182, 0x3a67, 0x87, 0xdf, 0xae, 0xd4, 0x90, 0xe1, 0x4d, 0xc6)]
interface _UIPermission(_UIPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission, IBuiltInPermission