// attributes.rs - MIT License
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

RIDL!{#[uuid(0x1b6ed26a, 0x4b7f, 0x34fc, 0xb2, 0xc8, 0x81, 0x09, 0xd6, 0x84, 0xb3, 0xdf)]
interface _UnmanagedFunctionPointerAttribute(_UnmanagedFunctionPointerAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbbe41ac5, 0x8692, 0x3427, 0x9a, 0xe1, 0xc1, 0x05, 0x8a, 0x38, 0xd4, 0x92)]
interface _DispIdAttribute(_DispIdAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

//enum __declspec(uuid("18c327e4-e4ba-3c3c-9942-274272626278"))
ENUM!{enum ComInterfaceType
{
    ComInterfaceType_InterfaceIsDual = 0,
    ComInterfaceType_InterfaceIsIUnknown = 1,
    ComInterfaceType_InterfaceIsIDispatch = 2,
}}

RIDL!{#[uuid(0xa2145f38, 0xcac1, 0x33dd, 0xa3, 0x18, 0x21, 0x94, 0x8a, 0xf6, 0x82, 0x5d)]
interface _InterfaceTypeAttribute(_InterfaceTypeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0c1e7b57, 0xb9b1, 0x36e4, 0x83, 0x96, 0x54, 0x9c, 0x29, 0x06, 0x2a, 0x81)]
interface _ComDefaultInterfaceAttribute(_ComDefaultInterfaceAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

//enum __declspec(uuid("d58dc4bb-3a4c-3b0c-b75f-9d0876694f3d"))
ENUM!{ enum ClassInterfaceType
{
    ClassInterfaceType_None = 0,
    ClassInterfaceType_AutoDispatch = 1,
    ClassInterfaceType_AutoDual = 2,
}}

RIDL!{#[uuid(0x6b6391ee, 0x842f, 0x3e9a, 0x8e, 0xee, 0xf1, 0x33, 0x25, 0xe1, 0x09, 0x96)]
interface _ClassInterfaceAttribute(_ClassInterfaceAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1e7fffe2, 0xaad9, 0x34ee, 0x8a, 0x9f, 0x3c, 0x01, 0x6b, 0x88, 0x0f, 0xf0)]
interface _ComVisibleAttribute(_ComVisibleAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x288a86d1, 0x6f4f, 0x39c9, 0x9e, 0x42, 0x16, 0x2c, 0xf1, 0xc3, 0x72, 0x26)]
interface _TypeLibImportClassAttribute(_TypeLibImportClassAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4ab67927, 0x3c86, 0x328a, 0x81, 0x86, 0xf8, 0x53, 0x57, 0xdd, 0x55, 0x27)]
interface _LCIDConversionAttribute(_LCIDConversionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x51ba926f, 0xaab5, 0x3945, 0xb8, 0xa6, 0xc8, 0xf0, 0xf4, 0xa7, 0xd1, 0x2b)]
interface _ComRegisterFunctionAttribute(_ComRegisterFunctionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9f164188, 0x34eb, 0x3f86, 0x9f, 0x74, 0x0b, 0xbe, 0x41, 0x55, 0xe6, 0x5e)]
interface _ComUnregisterFunctionAttribute(_ComUnregisterFunctionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2b9f01df, 0x5a12, 0x3688, 0x98, 0xd6, 0xc3, 0x4b, 0xf5, 0xed, 0x18, 0x65)]
interface _ProgIdAttribute(_ProgIdAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3f3311ce, 0x6baf, 0x3fb0, 0xb8, 0x55, 0x48, 0x9a, 0xff, 0x74, 0x0b, 0x6e)]
interface _ImportedFromTypeLibAttribute(_ImportedFromTypeLibAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

//enum __declspec(uuid("8a958a5b-626c-3d22-ab56-3ec30c9b7ee2"))
ENUM!{enum IDispatchImplType
{
    IDispatchImplType_SystemDefinedImpl = 0,
    IDispatchImplType_InternalImpl = 1,
    IDispatchImplType_CompatibleImpl = 2,
}}

RIDL!{#[uuid(0x5778e7c7, 0x2040, 0x330e, 0xb4, 0x7a, 0x92, 0x97, 0x4d, 0xff, 0xcf, 0xd4)]
interface _IDispatchImplAttribute(_IDispatchImplAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe1984175, 0x55f5, 0x3065, 0x82, 0xd8, 0xa6, 0x83, 0xfd, 0xfc, 0xf0, 0xac)]
interface _ComSourceInterfacesAttribute(_ComSourceInterfacesAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfd5b6aac, 0xff8c, 0x3472, 0xb8, 0x94, 0xcd, 0x6d, 0xfa, 0xdb, 0x69, 0x39)]
interface _ComConversionLossAttribute(_ComConversionLossAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

//enum __declspec(uuid("97aa3979-1066-3969-b278-e064bdb97dce"))
ENUM!{enum TypeLibTypeFlags
{
    TypeLibTypeFlags_FAppObject = 1,
    TypeLibTypeFlags_FCanCreate = 2,
    TypeLibTypeFlags_FLicensed = 4,
    TypeLibTypeFlags_FPreDeclId = 8,
    TypeLibTypeFlags_FHidden = 16,
    TypeLibTypeFlags_FControl = 32,
    TypeLibTypeFlags_FDual = 64,
    TypeLibTypeFlags_FNonExtensible = 128,
    TypeLibTypeFlags_FOleAutomation = 256,
    TypeLibTypeFlags_FRestricted = 512,
    TypeLibTypeFlags_FAggregatable = 1024,
    TypeLibTypeFlags_FReplaceable = 2048,
    TypeLibTypeFlags_FDispatchable = 4096,
    TypeLibTypeFlags_FReverseBind = 8192,
}}

//enum __declspec(uuid("bf1bf727-537f-3284-9ca9-5adf12641ab5"))
ENUM!{enum TypeLibFuncFlags
{
    TypeLibFuncFlags_FRestricted = 1,
    TypeLibFuncFlags_FSource = 2,
    TypeLibFuncFlags_FBindable = 4,
    TypeLibFuncFlags_FRequestEdit = 8,
    TypeLibFuncFlags_FDisplayBind = 16,
    TypeLibFuncFlags_FDefaultBind = 32,
    TypeLibFuncFlags_FHidden = 64,
    TypeLibFuncFlags_FUsesGetLastError = 128,
    TypeLibFuncFlags_FDefaultCollelem = 256,
    TypeLibFuncFlags_FUiDefault = 512,
    TypeLibFuncFlags_FNonBrowsable = 1024,
    TypeLibFuncFlags_FReplaceable = 2048,
    TypeLibFuncFlags_FImmediateBind = 4096,
}}

//enum __declspec(uuid("c660d7a6-d1dd-3e9d-85eb-f844791e2dae"))
ENUM!{enum TypeLibVarFlags
{
    TypeLibVarFlags_FReadOnly = 1,
    TypeLibVarFlags_FSource = 2,
    TypeLibVarFlags_FBindable = 4,
    TypeLibVarFlags_FRequestEdit = 8,
    TypeLibVarFlags_FDisplayBind = 16,
    TypeLibVarFlags_FDefaultBind = 32,
    TypeLibVarFlags_FHidden = 64,
    TypeLibVarFlags_FRestricted = 128,
    TypeLibVarFlags_FDefaultCollelem = 256,
    TypeLibVarFlags_FUiDefault = 512,
    TypeLibVarFlags_FNonBrowsable = 1024,
    TypeLibVarFlags_FReplaceable = 2048,
    TypeLibVarFlags_FImmediateBind = 4096,
}}

RIDL!{#[uuid(0xb5a1729e, 0xb721, 0x3121, 0xa8, 0x38, 0xfd, 0xe4, 0x3a, 0xf1, 0x34, 0x68)]
interface _TypeLibTypeAttribute(_TypeLibTypeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3d18a8e2, 0xeede, 0x3139, 0xb2, 0x9d, 0x8c, 0xac, 0x05, 0x79, 0x55, 0xdf)]
interface _TypeLibFuncAttribute(_TypeLibFuncAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7b89862a, 0x02a4, 0x3279, 0x8b, 0x42, 0x40, 0x95, 0xfa, 0x3a, 0x77, 0x8e)]
interface _TypeLibVarAttribute(_TypeLibVarAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

//skipped VarEnum

//enum __declspec(uuid("03d65b1a-bbf6-3bdc-bc53-85e02415670d"))
ENUM!{enum UnmanagedType
{
    UnmanagedType_Bool = 2,
    UnmanagedType_I1 = 3,
    UnmanagedType_U1 = 4,
    UnmanagedType_I2 = 5,
    UnmanagedType_U2 = 6,
    UnmanagedType_I4 = 7,
    UnmanagedType_U4 = 8,
    UnmanagedType_I8 = 9,
    UnmanagedType_U8 = 10,
    UnmanagedType_R4 = 11,
    UnmanagedType_R8 = 12,
    UnmanagedType_Currency = 15,
    UnmanagedType_BStr = 19,
    UnmanagedType_LPStr = 20,
    UnmanagedType_LPWStr = 21,
    UnmanagedType_LPTStr = 22,
    UnmanagedType_ByValTStr = 23,
    UnmanagedType_IUnknown = 25,
    UnmanagedType_IDispatch = 26,
    UnmanagedType_Struct = 27,
    UnmanagedType_Interface = 28,
    UnmanagedType_SafeArray = 29,
    UnmanagedType_ByValArray = 30,
    UnmanagedType_SysInt = 31,
    UnmanagedType_SysUInt = 32,
    UnmanagedType_VBByRefStr = 34,
    UnmanagedType_AnsiBStr = 35,
    UnmanagedType_TBStr = 36,
    UnmanagedType_VariantBool = 37,
    UnmanagedType_FunctionPtr = 38,
    UnmanagedType_AsAny = 40,
    UnmanagedType_LPArray = 42,
    UnmanagedType_LPStruct = 43,
    UnmanagedType_CustomMarshaler = 44,
    UnmanagedType_Error = 45,
}}

RIDL!{#[uuid(0xd858399f, 0xe19e, 0x3423, 0xa7, 0x20, 0xac, 0x12, 0xab, 0xe2, 0xe5, 0xe8)]
interface _MarshalAsAttribute(_MarshalAsAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1b093056, 0x5454, 0x386f, 0x89, 0x71, 0xbb, 0xcb, 0xc4, 0xe9, 0xa8, 0xf3)]
interface _ComImportAttribute(_ComImportAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x74435dad, 0xec55, 0x354b, 0x8f, 0x5b, 0xfa, 0x70, 0xd1, 0x3b, 0x62, 0x93)]
interface _GuidAttribute(_GuidAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfdf2a2ee, 0xc882, 0x3198, 0xa4, 0x8b, 0xe3, 0x7f, 0x0e, 0x57, 0x4d, 0xfa)]
interface _PreserveSigAttribute(_PreserveSigAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8474b65c, 0xc39a, 0x3d05, 0x89, 0x3d, 0x57, 0x7b, 0x9a, 0x31, 0x46, 0x15)]
interface _InAttribute(_InAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0697fc8c, 0x9b04, 0x3783, 0x95, 0xc7, 0x45, 0xec, 0xca, 0xc1, 0xca, 0x27)]
interface _OutAttribute(_OutAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0d6bd9ad, 0x198e, 0x3904, 0xad, 0x99, 0xf6, 0xf8, 0x2a, 0x27, 0x87, 0xc4)]
interface _OptionalAttribute(_OptionalAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa1a26181, 0xd55e, 0x3ee2, 0x96, 0xe6, 0x70, 0xb3, 0x54, 0xef, 0x93, 0x71)]
interface _DllImportAttribute(_DllImportAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x23753322, 0xc7b3, 0x3f9a, 0xac, 0x96, 0x52, 0x67, 0x2c, 0x1b, 0x1c, 0xa9)]
interface _StructLayoutAttribute(_StructLayoutAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc14342b8, 0xbafd, 0x322a, 0xbb, 0x71, 0x62, 0xc6, 0x72, 0xda, 0x28, 0x4e)]
interface _FieldOffsetAttribute(_FieldOffsetAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe78785c4, 0x3a73, 0x3c15, 0x93, 0x90, 0x61, 0x8b, 0xf3, 0xa1, 0x47, 0x19)]
interface _ComAliasNameAttribute(_ComAliasNameAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x57b908a8, 0xc082, 0x3581, 0x8a, 0x47, 0x6b, 0x41, 0xb8, 0x6e, 0x8f, 0xdc)]
interface _AutomationProxyAttribute(_AutomationProxyAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc69e96b2, 0x6161, 0x3621, 0xb1, 0x65, 0x58, 0x05, 0x19, 0x8c, 0x6b, 0x8d)]
interface _PrimaryInteropAssemblyAttribute(_PrimaryInteropAssemblyAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x15d54c00, 0x7c95, 0x38d7, 0xb8, 0x59, 0xe1, 0x93, 0x46, 0x67, 0x7d, 0xcd)]
interface _CoClassAttribute(_CoClassAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x76cc0491, 0x9a10, 0x35c0, 0x8a, 0x66, 0x79, 0x31, 0xec, 0x34, 0x5b, 0x7f)]
interface _ComEventInterfaceAttribute(_ComEventInterfaceAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa03b61a4, 0xca61, 0x3460, 0x82, 0x32, 0x2f, 0x4e, 0xc9, 0x6a, 0xa8, 0x8f)]
interface _TypeLibVersionAttribute(_TypeLibVersionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xad419379, 0x2ac8, 0x3588, 0xab, 0x1e, 0x01, 0x15, 0x41, 0x32, 0x77, 0xc4)]
interface _ComCompatibleVersionAttribute(_ComCompatibleVersionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xed47abe7, 0xc84b, 0x39f9, 0xbe, 0x1b, 0x82, 0x8c, 0xfb, 0x92, 0x5a, 0xfe)]
interface _BestFitMappingAttribute(_BestFitMappingAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb26b3465, 0x28e4, 0x33b5, 0xb9, 0xbf, 0xdd, 0x7c, 0x4f, 0x64, 0x61, 0xf5)]
interface _DefaultCharSetAttribute(_DefaultCharSetAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa54ac093, 0xbfce, 0x37b0, 0xa8, 0x1f, 0x14, 0x8d, 0xfe, 0xd0, 0x97, 0x1f)]
interface _SetWin32ContextInIDispatchAttribute(_SetWin32ContextInIDispatchAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}