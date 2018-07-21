use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::shared::guiddef::GUID;
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::SAFEARRAY;

use system::reflection::cominterfaces::_Type;
use system::reflection::cominterfaces::_Assembly;

//enum __declspec(uuid("765653a0-2b24-38e4-a6f6-5cb325e8ccc9"))
ENUM!{enum AssemblyRegistrationFlags
{
    AssemblyRegistrationFlags_None = 0,
    AssemblyRegistrationFlags_SetCodeBase = 1,
}}

RIDL!{#[uuid(0xccbd682c, 0x73a5, 0x4568, 0xb8, 0xb0, 0xc7, 0x00, 0x7e, 0x11, 0xab, 0xa2)]
interface IRegistrationServices(IRegistrationServicesVtbl): IDispatch(IDispatchVtbl)  
{
    fn RegisterAssembly(
        Assembly: *mut _Assembly,
        flags: AssemblyRegistrationFlags, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn UnregisterAssembly(
		Assembly: *mut  _Assembly ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn GetRegistrableTypesInAssembly(
		Assembly: *mut  _Assembly ,
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetProgIdForType(
		Type: *mut  _Type ,
		pRetVal: *mut BSTR ,
	) -> HRESULT,
    fn RegisterTypeForComClients(
        Type_: *mut _Type, 
        G: *mut GUID, 
    ) -> HRESULT,
    fn GetManagedCategoryGuid(
        pRetVal: GUID ,
    ) -> HRESULT,
    fn TypeRequiresRegistration(
		Type: *mut  _Type ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn TypeRepresentsComType(
		Type: *mut  _Type ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
}}