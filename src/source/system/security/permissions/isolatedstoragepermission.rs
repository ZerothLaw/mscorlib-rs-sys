// isolatedstoragepermission.rs - MIT License
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

//enum __declspec(uuid("0d6e31df-3a76-3054-a8eb-150e92300f89"))
ENUM!{enum IsolatedStorageContainment
{
    IsolatedStorageContainment_None = 0,
    IsolatedStorageContainment_DomainIsolationByUser = 16,
    IsolatedStorageContainment_ApplicationIsolationByUser = 21,
    IsolatedStorageContainment_AssemblyIsolationByUser = 32,
    IsolatedStorageContainment_DomainIsolationByMachine = 48,
    IsolatedStorageContainment_AssemblyIsolationByMachine = 64,
    IsolatedStorageContainment_ApplicationIsolationByMachine = 69,
    IsolatedStorageContainment_DomainIsolationByRoamingUser = 80,
    IsolatedStorageContainment_AssemblyIsolationByRoamingUser = 96,
    IsolatedStorageContainment_ApplicationIsolationByRoamingUser = 101,
    IsolatedStorageContainment_AdministerIsolatedStorageByUser = 112,
    IsolatedStorageContainment_UnrestrictedIsolatedStorage = 240,
}}

RIDL!{#[uuid(0x7fee7903, 0xf97c, 0x3350, 0xad, 0x42, 0x19, 0x6b, 0x00, 0xad, 0x25, 0x64)]
interface _IsolatedStoragePermission(_IsolatedStoragePermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission