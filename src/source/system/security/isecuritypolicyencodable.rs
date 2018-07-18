use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use source::system::security::policy::policylevel::_PolicyLevel;
use source::system::security::securityelement::_SecurityElement;

RIDL!{#[uuid(0xe6c21ba7, 0x21bb, 0x34e9, 0x8e, 0x57, 0xdb, 0x66, 0xd8, 0xce, 0x4a, 0x70)]
interface ISecurityPolicyEncodable(ISecurityPolicyEncodableVtbl): IDispatch(IDispatchVtbl)  
{
    fn ToXml(
		level: *mut  _PolicyLevel ,
		pRetVal: *mut *mut  _SecurityElement ,
	) -> HRESULT,
    fn FromXml(
        e: *mut _SecurityElement,
        level: *mut _PolicyLevel,
    ) -> HRESULT,
}}