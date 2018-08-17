// hostprotectionpermission.rs - MIT License
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

//enum __declspec(uuid("4548a129-2855-35e8-a892-ff506c877aa8"))
ENUM!{enum HostProtectionResource
{
    HostProtectionResource_None = 0,
    HostProtectionResource_Synchronization = 1,
    HostProtectionResource_SharedState = 2,
    HostProtectionResource_ExternalProcessMgmt = 4,
    HostProtectionResource_SelfAffectingProcessMgmt = 8,
    HostProtectionResource_ExternalThreading = 16,
    HostProtectionResource_SelfAffectingThreading = 32,
    HostProtectionResource_SecurityInfrastructure = 64,
    HostProtectionResource_UI = 128,
    HostProtectionResource_MayLeakOnAbort = 256,
    HostProtectionResource_All = 511,
}}

RIDL!{#[uuid(0x9f8f73a3, 0x1e99, 0x3e51, 0xa4, 0x1b, 0x17, 0x9a, 0x41, 0xdc, 0x74, 0x7c)]
interface _HostProtectionAttribute(_HostProtectionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute