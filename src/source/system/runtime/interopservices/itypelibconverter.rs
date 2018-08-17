//    Copyright 2018 Tyler Laing
// 
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
// 
//        http://www.apache.org/licenses/LICENSE-2.0
// 
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::SAFEARRAY;
use winapi::ctypes::c_long;
use winapi::shared::wtypes::BSTR;
use winapi::um::winnt::LCID;
use winapi::shared::guiddef::GUID;

use system::reflection::_Assembly;
use system::reflection::emit::_AssemblyBuilder;
use system::version::_Version;
use system::reflection::_StrongNameKeyPair;


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
