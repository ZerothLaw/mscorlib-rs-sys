// iappdomain.rs - MIT License
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

//.Net Framework 4.7.2 Reference Source - mscorlib { system.iappdomain.cs }
//https://referencesource.microsoft.com/#mscorlib/system/iappdomain.cs,e55c0aa25d0b566c
use winapi::ctypes::{c_long};

use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::{WORD,UINT};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};
use winapi::shared::wtypesbase::LPOLESTR;

use winapi::um::oaidl::{DISPID, DISPPARAMS, EXCEPINFO, ITypeInfo, VARIANT, SAFEARRAY};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

use crate::system::{_AssemblyLoadEventHandler, _CrossAppDomainDelegate, _EventHandler, _ResolveEventHandler, _UnhandledExceptionEventHandler};

use crate::system::globalization::_CultureInfo;

use crate::system::reflection::{_Assembly, _AssemblyName, _Binder, _Type};
use crate::system::reflection::BindingFlags;

use crate::system::reflection::emit::_AssemblyBuilder;
use crate::system::reflection::emit::AssemblyBuilderAccess;

use crate::system::security::_PermissionSet;
use crate::system::security::principal::{IPrincipal,PrincipalPolicy};
use crate::system::security::policy::{_Evidence,_PolicyLevel};
use crate::system::threading::IObjectHandle;

//RIDL!{#[uuid()]}
//"([\w\d]{8})-([\w\d]{4})-([\w\d]{4})-([\w\d]{2})([\w\d]{2})-([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})"
//0x$1, 0x$2, 0x$3, 0x$4, 0x$5, 0x$6, 0x$7, 0x$8, 0x$9, 0x$10, 0x$11

STRUCT!{struct ResolveEventArgs{
    _Name: BSTR,
    _RequestingAssembly: *mut _Assembly,
}}

STRUCT!{struct AssemblyLoadEventArgs{
    _LoadedAssembly: *mut _Assembly,
}}

// type ResolveEventHandler = dyn Fn(*mut VARIANT, ResolveEventArgs) -> *mut _Assembly;
// type AssemblyLoadEventHandler = dyn Fn(*mut VARIANT, AssemblyLoadEventArgs);
// type AppDomainInitializer = dyn Fn(*mut SAFEARRAY); //string[]
// type CrossAppDomainDelegate = dyn Fn();

RIDL!{#[uuid(0x05F696DC, 0x2B29, 0x3663, 0xAD, 0x8B, 0xC4, 0x38, 0x9C, 0xF2, 0xA7, 0x13)]
interface _AppDomain(_AppDomainVtbl): IUnknown(IUnknownVtbl){
    fn GetTypeInfoCount( //always throws NotImplementedException
        pcTInfo: *mut UINT, 
    ) -> HRESULT, 
    fn GetTypeInfo( //always throws NotImplementedException
        iTInfo: UINT, 
        lcid: UINT, 
        ppTInfo: *mut *mut ITypeInfo, 
    ) -> HRESULT, 
    fn GetIDsOfNames( //always throws NotImplementedException
        riid: REFIID, 
        rgszNames: *mut LPOLESTR, 
        cNames: UINT, 
        lcid: UINT, 
        rgDispId: *mut DISPID,
    ) -> HRESULT,
    fn Invoke( //always throws NotImplementedException
        dispIdMember: DISPID,
        riid: REFIID, 
        wFlags: WORD, 
        pDispParams: *mut DISPPARAMS,
        pVarResult: *mut VARIANT, 
        pExcepInfo: *mut EXCEPINFO,
        puArgError: *mut UINT,
    ) -> HRESULT,
    fn ToString_(
        pRetVal: *mut BSTR, 
    ) -> HRESULT,
    fn Equals(
        other: VARIANT, 
        pRetVal: *mut VARIANT_BOOL, 
    ) -> HRESULT, 
    fn GetHashCode(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn GetType(
        pRetVal: *mut *mut _Type,
    )->HRESULT,
    fn InitializeLifetimeService(
        pRetVal: *mut *mut VARIANT, 
    ) -> HRESULT, 
    fn GetLifetimeService(
        pRetVal: *mut *mut VARIANT, 
    ) -> HRESULT,
    fn get_Evidence(
        pRetVal: *mut *mut _Evidence,
    ) -> HRESULT,
    fn set_Evidence(
        pRetVal: *mut _Evidence,
    ) -> HRESULT,
    fn get_DomainUnload(
        pRetVal: *mut *mut _EventHandler,
    ) -> HRESULT,
    fn set_DomainUnload(
        pRetVal: *mut _EventHandler,
    ) -> HRESULT,
    fn get_AssemblyLoad(
        pRetVal: *mut *mut _AssemblyLoadEventHandler,
    ) -> HRESULT,
    fn set_AssemblyLoad(
        pRetVal: *mut _AssemblyLoadEventHandler,
    ) -> HRESULT,
    fn get_ProcessExit(
        pRetVal: *mut *mut _EventHandler,
    ) -> HRESULT,
    fn set_ProcessExit(
        pRetVal: *mut _EventHandler,
    ) -> HRESULT,
    fn get_TypeResolve(
        pRetVal: *mut *mut _ResolveEventHandler,
    ) -> HRESULT,
    fn set_TypeResolve(
        pRetVal: *mut _ResolveEventHandler,
    ) -> HRESULT,
    fn get_ResourceResolve(
        pRetVal: *mut *mut _ResolveEventHandler,
    ) -> HRESULT,
    fn set_ResourceResolve(
        pRetVal: *mut _ResolveEventHandler,
    ) -> HRESULT,
    fn get_AssemblyResolve(
        pRetVal: *mut *mut _ResolveEventHandler,
    ) -> HRESULT,
    fn get_UnhandledException(
        pRetVal: *mut *mut _UnhandledExceptionEventHandler,
    ) -> HRESULT,
    fn set_UnhandledException(
        pRetVal: *mut _UnhandledExceptionEventHandler,
    ) -> HRESULT,
    fn DefineDynamicAssembly(
        name: *mut _AssemblyName, 
        access: AssemblyBuilderAccess, //enum
        pRetVal: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn DefineDynamicAssembly_2(
        name: *mut _AssemblyName, 
        access: AssemblyBuilderAccess, //enum
        dir: BSTR,
        pRetVal: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn DefineDynamicAssembly_3(
        name: *mut _AssemblyName, 
        access: AssemblyBuilderAccess, //enum
        evidence: *mut _Evidence,
        pRetVal: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn DefineDynamicAssembly_4(
        name: *mut _AssemblyName, 
        access: AssemblyBuilderAccess, //enum
        requiredPermissions: _PermissionSet, 
        optionalPermissions: _PermissionSet, 
        refusedPermissions: _PermissionSet,
        pRetVal: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn DefineDynamicAssembly_5( //obsolete
        name: *mut _AssemblyName, 
        access: AssemblyBuilderAccess, //enum
        dir: BSTR,
        evidence: *mut _Evidence,
        pRetVal: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn DefineDynamicAssembly_6( //obsolete
        name: *mut _AssemblyName, 
        access: AssemblyBuilderAccess, //enum
        dir: BSTR,
        requiredPermissions: _PermissionSet, 
        optionalPermissions: _PermissionSet, 
        refusedPermissions: _PermissionSet,
        pRetVal: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn DefineDynamicAssembly_7( //obsolete
        name: *mut _AssemblyName, 
        access: AssemblyBuilderAccess, //enum
        evidence: *mut _Evidence,
        requiredPermissions: _PermissionSet, 
        optionalPermissions: _PermissionSet, 
        refusedPermissions: _PermissionSet,
        pRetVal: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn DefineDynamicAssembly_8( //obsolete
        name: *mut _AssemblyName, 
        access: AssemblyBuilderAccess, //enum
        dir: BSTR,
        evidence: *mut _Evidence,
        requiredPermissions: _PermissionSet, 
        optionalPermissions: _PermissionSet, 
        refusedPermissions: _PermissionSet,
        pRetVal: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn DefineDynamicAssembly_9( //obsolete
        name: *mut _AssemblyName, 
        access: AssemblyBuilderAccess, //enum
        dir: BSTR,
        evidence: *mut _Evidence,
        requiredPermissions: _PermissionSet, 
        optionalPermissions: _PermissionSet, 
        refusedPermissions: _PermissionSet,
        isSynchronized: VARIANT,
        pRetVal: *mut *mut _AssemblyBuilder, 
    ) -> HRESULT,
    fn CreateInstance(
        assemblyName: BSTR, 
        typeName: BSTR,
        pRetVal: *mut *mut IObjectHandle, //?? ObjectHandle in c#
    ) -> HRESULT,
    fn CreateInstanceFrom(
        assemblyFile: BSTR, 
        typeName: BSTR, 
        pRetVal: *mut *mut IObjectHandle,
    ) -> HRESULT, 
    fn CreateInstance_2(
        assemblyName: BSTR, 
        typeName: BSTR, 
        activationAttributes: *mut SAFEARRAY, //Object[]
        pRetVal: *mut *mut IObjectHandle,
    ) -> HRESULT, 
    fn CreateInstanceFrom_2(
        assemblyFile: BSTR, 
        typeName: BSTR, 
        activationAttributes: *mut SAFEARRAY, //Object[]
        pRetVal: *mut *mut IObjectHandle,
    ) -> HRESULT, 
    fn CreateInstance_3(
        assemblyName: BSTR, 
        typeName: BSTR, 
        ignoreCase: VARIANT_BOOL, 
        bindingAttr: BindingFlags,
        binder: *mut _Binder, 
        args: *mut SAFEARRAY,
        culture: *mut _CultureInfo,
        activationAttributes: *mut SAFEARRAY, //Object[]
        securityAttributes: *mut _Evidence,
        pRetVal: *mut *mut IObjectHandle,
    ) -> HRESULT, 
    fn CreateInstanceFrom_3(
        assemblyFile: BSTR, 
        typeName: BSTR, 
        ignoreCase: VARIANT_BOOL, 
        bindingAttr: BindingFlags,
        binder: *mut _Binder, 
        args: *mut SAFEARRAY,
        culture: *mut _CultureInfo,
        activationAttributes: *mut SAFEARRAY, //Object[]
        securityAttributes: *mut _Evidence,
        pRetVal: *mut *mut IObjectHandle,
    ) -> HRESULT, 
    fn Load(
        assemblyRef: *mut _AssemblyName, 
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT,
    fn Load_2(
        assemblyString: BSTR, 
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT,
    fn Load_3(
        rawAssembly: *mut SAFEARRAY, //byte[]
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT,
    fn Load_4(
        rawAssembly: *mut SAFEARRAY, //byte[]
        rawSymbolStore: *mut SAFEARRAY, //byte[]
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT,
    fn Load_5(
        rawAssembly: *mut SAFEARRAY, //byte[]
        rawSymbolStore: *mut SAFEARRAY, //byte[]
        securityEvidence: *mut _Evidence,
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT,
    fn Load_6(
        assemblyRef: *mut _AssemblyName, 
        assemblySecurity: *mut _Evidence,
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT,
    fn Load_7(
        assemblyString: BSTR, 
        assemblySecurity: *mut _Evidence,
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT,
    fn ExecuteAssembly(
        assemblyFile: BSTR, 
        assemblySecurity: *mut _Evidence, 
        pRetVal: *mut c_long,
    ) -> HRESULT, 
    fn ExecuteAssembly_2(
        assemblyFile: BSTR,  
        pRetVal: *mut c_long,
    ) -> HRESULT, 
    fn ExecuteAssembly_3(
        assemblyFile: BSTR, 
        assemblySecurity: *mut _Evidence, 
        args: *mut SAFEARRAY, //String[]
        pRetVal: *mut c_long,
    ) -> HRESULT, 
    fn get_FriendlyName(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn get_BaseDirectory(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn get_RelativeSearchPath(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn get_ShadowCopyFiles(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetAssemblies(
        pRetVal: *mut *mut SAFEARRAY, //Assembly[]
    ) -> HRESULT,
    fn AppendPrivatePath(
        path: BSTR, 
    ) -> HRESULT,
    fn ClearPrivatePath() -> HRESULT,
    fn SetShadowCopyPath(
        s: BSTR, 
    ) -> HRESULT, 
    fn ClearShadowCopyPath( ) -> HRESULT,
    fn SetCachePath(
        s: BSTR, 
    ) -> HRESULT,
    fn SetData(
        name: BSTR, 
        data: VARIANT, //object
    ) -> HRESULT,
    fn GetData(
        name: BSTR, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn SetAppDomainPolicy(
        domainPolicy: *mut _PolicyLevel,
    ) -> HRESULT,
    fn SetThreadPrincipal(
        principal: *mut IPrincipal, 
    ) -> HRESULT, 
    fn SetPrincipalPolicy(
        policy: *mut PrincipalPolicy, 
    ) -> HRESULT, 
    fn DoCallBack(
        theDelegate: _CrossAppDomainDelegate,
    ) -> HRESULT, 
    fn get_DynamicDirectory(
        pRetVal: *mut BSTR, 
    ) -> HRESULT,
}}