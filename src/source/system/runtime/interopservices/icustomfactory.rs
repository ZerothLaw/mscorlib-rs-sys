use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use system::reflection::_Type;

use system::_MarshalByRefObject;

RIDL!{#[uuid(0x0ca9008e, 0xee90, 0x356e, 0x9f, 0x6d, 0xb5, 0x9e, 0x60, 0x06, 0xb9, 0xa4)]
interface ICustomFactory(ICustomFactoryVtbl): IDispatch(IDispatchVtbl)  
{
    fn CreateInstance(
		serverType: *mut  _Type,
		pRetVal: *mut *mut  _MarshalByRefObject,
	) -> HRESULT,
}}