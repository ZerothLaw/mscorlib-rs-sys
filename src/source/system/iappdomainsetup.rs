//.Net Framework 4.7.2 Reference Source - mscorlib { system.iappdomainsetup.cs }

//RIDL!{#[uuid()]}
//"([\w\d]{8})-([\w\d]{4})-([\w\d]{4})-([\w\d]{2})([\w\d]{2})-([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})([\w\d]{2})"
//0x$1, 0x$2, 0x$3, 0x$4, 0x$5, 0x$6, 0x$7, 0x$8, 0x$9, 0x$10, 0x$11

use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;

use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};

RIDL!{#[uuid(0x27FFF232, 0xA7A8, 0x40dd, 0x8D, 0x4A, 0x73, 0x4A, 0xD5, 0x9F, 0xCD, 0x41)]
interface IAppDomainSetup(IAppDomainSetupVtbl): IUnknown(IUnknownVtbl){
    fn get_ApplicationBase(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn set_ApplicationBase(
        appBase: BSTR, 
    ) -> HRESULT,
    fn get_ApplicationName(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn set_ApplicationName(
        appBase: BSTR, 
    ) -> HRESULT,
    fn get_CachePath(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn set_CachePath(
        appBase: BSTR, 
    ) -> HRESULT,
    fn get_ConfigurationFile(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn set_ConfigurationFile(
        appBase: BSTR, 
    ) -> HRESULT,
    fn get_DynamicBase(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn set_DynamicBase(
        appBase: BSTR, 
    ) -> HRESULT,
    fn get_LicenseFile(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn set_LicenseFile(
        appBase: BSTR, 
    ) -> HRESULT,
    fn get_PrivateBinPath(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn set_PrivateBinPath(
        appBase: BSTR, 
    ) -> HRESULT,
    fn get_PrivateBinPathProbe(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn set_PrivateBinPathProbe(
        appBase: BSTR, 
    ) -> HRESULT,
    fn get_ShadowCopyDirectories(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn set_ShadowCopyDirectories(
        appBase: BSTR, 
    ) -> HRESULT,
    fn get_ShadowCopyFiles(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 
    fn set_ShadowCopyFiles(
        appBase: BSTR, 
    ) -> HRESULT,
}}