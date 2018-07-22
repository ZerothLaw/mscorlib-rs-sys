use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use system::security::securityelement::_SecurityElement;

RIDL!{#[uuid(0xfd46bde5, 0xacdf, 0x3ca5, 0xb1, 0x89, 0xf0, 0x67, 0x83, 0x87, 0x07, 0x7f)]
interface ISecurityEncodable(ISecurityEncodableVtbl): IDispatch(IDispatchVtbl)  
{
    fn ToXml(
		pRetVal: *mut *mut  _SecurityElement ,
	) -> HRESULT,
    fn FromXml(
        e: *mut _SecurityElement,
    ) -> HRESULT,
}}