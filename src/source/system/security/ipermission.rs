use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;

RIDL!{#[uuid(0xa19b3fc6, 0xd680, 0x3dd4, 0xa1, 0x7a, 0xf5, 0x8a, 0x7d, 0x48, 0x14, 0x94)]
interface IPermission(IPermissionVtbl): IDispatch(IDispatchVtbl)  
{
    fn Copy(
	    pRetVal: *mut *mut IPermission ,
	) -> HRESULT,
    fn Intersect(
		Target: *mut IPermission ,
		pRetVal: *mut *mut  IPermission ,
    ) -> HRESULT,
    fn Union(
	    Target: *mut IPermission ,
		pRetVal: *mut *mut IPermission ,
	) -> HRESULT,
    fn IsSubsetOf(
	    Target: *mut  IPermission ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn Demand() -> HRESULT,
}} //ISecurityEncodable