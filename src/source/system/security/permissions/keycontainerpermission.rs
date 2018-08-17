// keycontainerpermission.rs - MIT License
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

//enum __declspec(uuid("742bdc16-f04e-3e0e-8ff1-e3250940b5bf"))
ENUM!{enum KeyContainerPermissionFlags
{
    KeyContainerPermissionFlags_NoFlags = 0,
    KeyContainerPermissionFlags_Create = 1,
    KeyContainerPermissionFlags_Open = 2,
    KeyContainerPermissionFlags_Delete = 4,
    KeyContainerPermissionFlags_Import = 16,
    KeyContainerPermissionFlags_Export = 32,
    KeyContainerPermissionFlags_Sign = 256,
    KeyContainerPermissionFlags_Decrypt = 512,
    KeyContainerPermissionFlags_ViewAcl = 4096,
    KeyContainerPermissionFlags_ChangeAcl = 8192,
    KeyContainerPermissionFlags_AllFlags = 13111,
}}

RIDL!{#[uuid(0x094351ea, 0xdbc1, 0x327f, 0x8a, 0x83, 0x91, 0x3b, 0x59, 0x3a, 0x66, 0xbe)]
interface _KeyContainerPermissionAccessEntry(_KeyContainerPermissionAccessEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x28ecf94e, 0x3510, 0x3a3e, 0x8b, 0xd1, 0xf8, 0x66, 0xf4, 0x5f, 0x3b, 0x06)]
interface _KeyContainerPermissionAccessEntryCollection(_KeyContainerPermissionAccessEntryCollectionVtbl): IDispatch(IDispatchVtbl)  
{}} //ICollection

RIDL!{#[uuid(0x293187ea, 0x5f88, 0x316f, 0x86, 0xa5, 0x53, 0x3b, 0x0c, 0x7b, 0x35, 0x3f)]
interface _KeyContainerPermissionAccessEntryEnumerator(_KeyContainerPermissionAccessEntryEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}} //IEnumerator 

RIDL!{#[uuid(0x107a3cf1, 0xb35e, 0x3a23, 0xb6, 0x60, 0x60, 0x26, 0x4b, 0x23, 0x12, 0x25)]
interface _KeyContainerPermission(_KeyContainerPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission, IBuiltInPermission 
