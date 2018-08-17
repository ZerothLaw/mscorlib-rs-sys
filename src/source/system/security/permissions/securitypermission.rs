// securitypermission.rs - MIT License
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

//enum __declspec(uuid("b718f0f8-e5e7-3651-a2be-97009b568250"))
ENUM!{enum SecurityPermissionFlag
{
    SecurityPermissionFlag_NoFlags = 0,
    SecurityPermissionFlag_Assertion = 1,
    SecurityPermissionFlag_UnmanagedCode = 2,
    SecurityPermissionFlag_SkipVerification = 4,
    SecurityPermissionFlag_Execution = 8,
    SecurityPermissionFlag_ControlThread = 16,
    SecurityPermissionFlag_ControlEvidence = 32,
    SecurityPermissionFlag_ControlPolicy = 64,
    SecurityPermissionFlag_SerializationFormatter = 128,
    SecurityPermissionFlag_ControlDomainPolicy = 256,
    SecurityPermissionFlag_ControlPrincipal = 512,
    SecurityPermissionFlag_ControlAppDomain = 1024,
    SecurityPermissionFlag_RemotingConfiguration = 2048,
    SecurityPermissionFlag_Infrastructure = 4096,
    SecurityPermissionFlag_BindingRedirects = 8192,
    SecurityPermissionFlag_AllFlags = 16383,
}}

RIDL!{#[uuid(0x33c54a2d, 0x02bd, 0x3848, 0x80, 0xb6, 0x74, 0x2d, 0x53, 0x70, 0x85, 0xe5)]
interface _SecurityPermission(_SecurityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission, IBuiltInPermission