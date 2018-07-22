use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use system::security::policy::evidence::_Evidence;
use system::security::ipermission::IPermission;

RIDL!{#[uuid(0x4e95244e, 0xc6fc, 0x3a86, 0x8d, 0xb7, 0x17, 0x12, 0x45, 0x4d, 0xe3, 0xb6)]
interface IIdentityPermissionFactory(IIdentityPermissionFactoryVtbl): IDispatch(IDispatchVtbl)  
{
    fn CreateIdentityPermission(
		Evidence: *mut  _Evidence ,
		pRetVal: *mut *mut  IPermission ,
	) -> HRESULT,
}}