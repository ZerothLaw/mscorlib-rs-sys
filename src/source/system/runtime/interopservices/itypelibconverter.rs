// itypelibconverter.rs - MIT License
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

use winapi::ctypes::c_long;
use winapi::shared::guiddef::GUID;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};

use winapi::um::oaidl::SAFEARRAY;
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::LCID;

use crate::system::reflection::{_Assembly,_StrongNameKeyPair};
use crate::system::reflection::emit::_AssemblyBuilder;
use crate::system::version::_Version;

//enum __declspec(uuid("c335350a-892d-37f7-967c-99b3c4c4a301"))
ENUM!{enum TypeLibImporterFlags
{
    TypeLibImporterFlags_None = 0,
    TypeLibImporterFlags_PrimaryInteropAssembly = 1,
    TypeLibImporterFlags_UnsafeInterfaces = 2,
    TypeLibImporterFlags_SafeArrayAsSystemArray = 4,
    TypeLibImporterFlags_TransformDispRetVals = 8,
    TypeLibImporterFlags_PreventClassMembers = 16,
    TypeLibImporterFlags_SerializableValueClasses = 32,
    TypeLibImporterFlags_ImportAsX86 = 256,
    TypeLibImporterFlags_ImportAsX64 = 512,
    TypeLibImporterFlags_ImportAsItanium = 1024,
    TypeLibImporterFlags_ImportAsAgnostic = 2048,
    TypeLibImporterFlags_ReflectionOnlyLoading = 4096,
    TypeLibImporterFlags_NoDefineVersionResource = 8192,
    TypeLibImporterFlags_ImportAsArm = 16384,
}}

//enum __declspec(uuid("ad92602f-55f2-3552-a977-d93c79db346e"))
ENUM!{enum TypeLibExporterFlags
{
    TypeLibExporterFlags_None = 0,
    TypeLibExporterFlags_OnlyReferenceRegistered = 1,
    TypeLibExporterFlags_CallerResolvedReferences = 2,
    TypeLibExporterFlags_OldNames = 4,
    TypeLibExporterFlags_ExportAs32Bit = 16,
    TypeLibExporterFlags_ExportAs64Bit = 32,
}}


//enum __declspec(uuid("b42619b4-0edc-3f55-aa64-2140275fa115"))
ENUM!{enum ImporterEventKind
{
    ImporterEventKind_NOTIF_TYPECONVERTED = 0,
    ImporterEventKind_NOTIF_CONVERTWARNING = 1,
    ImporterEventKind_ERROR_REFTOINVALIDTYPELIB = 2,
}}

//enum __declspec(uuid("26170123-45fd-30f7-987d-bf3689662b6c"))
ENUM!{enum ExporterEventKind
{
    ExporterEventKind_NOTIF_TYPECONVERTED = 0,
    ExporterEventKind_NOTIF_CONVERTWARNING = 1,
    ExporterEventKind_ERROR_REFTOINVALIDASSEMBLY = 2,
}}

RIDL!{#[uuid(0xf1c3bf77, 0xc3e4, 0x11d3, 0x88, 0xe7, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface ITypeLibExporterNotifySink(ITypeLibExporterNotifySinkVtbl): IUnknown(IUnknownVtbl)   
{
    fn InteropServices_ReportEvent(
        eventKind: ExporterEventKind,
        eventCode: c_long,
        eventMsg: BSTR,
    ) -> HRESULT,
    fn ResolveRef(
        Assembly: *mut _Assembly,
        pRetVal: *mut *mut IUnknown,
    ) -> HRESULT,
}}


RIDL!{#[uuid(0xf1c3bf76, 0xc3e4, 0x11d3, 0x88, 0xe7, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface ITypeLibImporterNotifySink(ITypeLibImporterNotifySinkVtbl): IUnknown(IUnknownVtbl)  
{
    fn InteropServices_ReportEvent(
        eventKind: ImporterEventKind, 
        eventCode: c_long, 
        eventMsg: BSTR, 
    ) -> HRESULT,
    fn ResolveRef(
		typeLib: *mut IUnknown ,
		pRetVal: *mut *mut  _Assembly ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0xf1c3bf78, 0xc3e4, 0x11d3, 0x88, 0xe7, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface ITypeLibConverter(ITypeLibConverterVtbl): IUnknown(IUnknownVtbl)   
{
    fn ConvertTypeLibToAssembly(
        typeLib: *mut IUnknown, 
        asmFileName: BSTR, 
        flags: TypeLibImporterFlags, 
        notifySink: *mut ITypeLibImporterNotifySink, 
        publicKey: *mut SAFEARRAY,
        keyPair: *mut _StrongNameKeyPair, 
        asmNamespace: BSTR,
        asmVersion: *mut _Version, 
        pRetVal: *mut *mut _AssemblyBuilder,
    ) -> HRESULT,
    fn ConvertAssemblyToTypeLib(
        assembly: *mut _Assembly,
        typeLibName: BSTR, 
        flags: TypeLibExporterFlags, 
        notifySink: *mut ITypeLibExporterNotifySink,
        pRetVal: *mut *mut IUnknown,
    ) -> HRESULT,
    fn GetPrimaryInteropAssembly(
        G: GUID, 
        major: c_long, 
        minor: c_long, 
        lcid: LCID, 
        asmName: *mut BSTR, 
        asmCodeBase: *mut BSTR, 
        pRetVal: *mut VARIANT_BOOL, 
    ) -> HRESULT, 
    fn ConvertTypeLibToAssembly_2(
        typeLib: *mut IUnknown, 
        asmFileName: BSTR, 
        flags: c_long, 
        notifySink: *mut ITypeLibImporterNotifySink, 
        publicKey: *mut SAFEARRAY,
        keyPair: *mut _StrongNameKeyPair, 
        unsafeInterfaces: VARIANT_BOOL, 
        pRetVal: *mut *mut _AssemblyBuilder,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xfa1f3615, 0xacb9, 0x486d, 0x9e, 0xac, 0x1b, 0xef, 0x87, 0xe3, 0x6b, 0x09)]
interface ITypeLibExporterNameProvider(ITypeLibExporterNameProviderVtbl) : IUnknown(IUnknownVtbl)
{
    fn GetNames(
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}
