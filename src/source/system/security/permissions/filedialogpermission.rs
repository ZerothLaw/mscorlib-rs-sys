// filedialogpermission.rs - MIT License
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

//enum __declspec(uuid("0df04a9b-dddc-3777-a6b1-9604b5ced191"))
ENUM!{enum FileDialogPermissionAccess
{
    FileDialogPermissionAccess_None = 0,
    FileDialogPermissionAccess_Open = 1,
    FileDialogPermissionAccess_Save = 2,
    FileDialogPermissionAccess_OpenSave = 3,
}}

RIDL!{#[uuid(0xa8b7138c, 0x8932, 0x3d78, 0xa5, 0x85, 0xa9, 0x15, 0x69, 0xc7, 0x43, 0xac)]
interface _FileDialogPermission(_FileDialogPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}