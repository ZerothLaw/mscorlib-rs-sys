// permissionattributes.rs - MIT License
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

//enum __declspec(uuid("ba99ae52-d539-362f-b78c-4e84c14158bf"))
ENUM!{enum SecurityAction
{
    SecurityAction_Demand = 2,
    SecurityAction_Assert = 3,
    SecurityAction_Deny = 4,
    SecurityAction_PermitOnly = 5,
    SecurityAction_LinkDemand = 6,
    SecurityAction_InheritanceDemand = 7,
    SecurityAction_RequestMinimum = 8,
    SecurityAction_RequestOptional = 9,
    SecurityAction_RequestRefuse = 10,
}}

RIDL!{#[uuid(0x48815668, 0x6c27, 0x3312, 0x80, 0x3e, 0x27, 0x57, 0xf5, 0x5c, 0xe9, 0x6a)]
interface _SecurityAttribute(_SecurityAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //System.Attribute

RIDL!{#[uuid(0x9c5149cb, 0xd3c6, 0x32fd, 0xa0, 0xd5, 0x95, 0x35, 0x0d, 0xe7, 0xb8, 0x13)]
interface _CodeAccessSecurityAttribute(_CodeAccessSecurityAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //SecurityAttribute

RIDL!{#[uuid(0x4164071a, 0xed12, 0x3bdd, 0xaf, 0x40, 0xfd, 0xab, 0xca, 0xa7, 0x7d, 0x5f)]
interface _EnvironmentPermissionAttribute(_EnvironmentPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0x0ccca629, 0x440f, 0x313e, 0x96, 0xcd, 0xba, 0x1b, 0x4b, 0x49, 0x97, 0xf7)]
interface _FileDialogPermissionAttribute(_FileDialogPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0x0dca817d, 0xf21a, 0x3943, 0xb5, 0x4c, 0x5e, 0x80, 0x0c, 0xe5, 0xbc, 0x50)]
interface _FileIOPermissionAttribute(_FileIOPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0xedb51d1c, 0x08ad, 0x346a, 0xbe, 0x6f, 0xd7, 0x4f, 0xd6, 0xd6, 0xf9, 0x65)]
interface _KeyContainerPermissionAttribute(_KeyContainerPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0x68ab69e4, 0x5d68, 0x3b51, 0xb7, 0x4d, 0x1b, 0xea, 0xb9, 0xf3, 0x7f, 0x2b)]
interface _PrincipalPermissionAttribute(_PrincipalPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0xd31eed10, 0xa5f0, 0x308f, 0xa9, 0x51, 0xe5, 0x57, 0x96, 0x1e, 0xc5, 0x68)]
interface _ReflectionPermissionAttribute(_ReflectionPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0x38b6068c, 0x1e94, 0x3119, 0x88, 0x41, 0x1e, 0xca, 0x35, 0xed, 0x85, 0x78)]
interface _RegistryPermissionAttribute(_RegistryPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0x3a5b876c, 0xcde4, 0x32d2, 0x9c, 0x7e, 0x02, 0x0a, 0x14, 0xac, 0xa3, 0x32)]
interface _SecurityPermissionAttribute(_SecurityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0x1d5c0f70, 0xaf29, 0x38a3, 0x94, 0x36, 0x30, 0x70, 0xa3, 0x10, 0xc7, 0x3b)]
interface _UIPermissionAttribute(_UIPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0x2e3be3ed, 0x2f22, 0x3b20, 0x9f, 0x92, 0xbd, 0x29, 0xb7, 0x9d, 0x6f, 0x42)]
interface _ZoneIdentityPermissionAttribute(_ZoneIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0xc9a740f4, 0x26e9, 0x39a8, 0x88, 0x85, 0x8c, 0xa2, 0x6b, 0xd7, 0x9b, 0x21)]
interface _StrongNameIdentityPermissionAttribute(_StrongNameIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0x6fe6894a, 0x2a53, 0x3fb6, 0xa0, 0x6e, 0x34, 0x8f, 0x9b, 0xda, 0xd2, 0x3b)]
interface _SiteIdentityPermissionAttribute(_SiteIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0xca4a2073, 0x48c5, 0x3e61, 0x83, 0x49, 0x11, 0x70, 0x1a, 0x90, 0xdd, 0x9b)]
interface _UrlIdentityPermissionAttribute(_UrlIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0x6722c730, 0x1239, 0x3784, 0xac, 0x94, 0xc2, 0x85, 0xae, 0x5b, 0x90, 0x1a)]
interface _PublisherIdentityPermissionAttribute(_PublisherIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0x5c4c522f, 0xde4e, 0x3595, 0x9a, 0xa9, 0x93, 0x19, 0xc8, 0x6a, 0x52, 0x83)]
interface _IsolatedStoragePermissionAttribute(_IsolatedStoragePermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0x6f1f8aae, 0xd667, 0x39cc, 0x98, 0xfa, 0x72, 0x2b, 0xeb, 0xbb, 0xea, 0xc3)]
interface _IsolatedStorageFilePermissionAttribute(_IsolatedStorageFilePermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //IsolatedStoragePermissionAttribute

RIDL!{#[uuid(0x947a1995, 0xbc16, 0x3e7c, 0xb6, 0x5a, 0x99, 0xe7, 0x1f, 0x39, 0xc0, 0x91)]
interface _PermissionSetAttribute(_PermissionSetAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute