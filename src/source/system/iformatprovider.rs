use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::VARIANT;

use system::reflection::_Type;

RIDL!{#[uuid(0xc8cb1ded, 0x2814, 0x396a, 0x9c, 0xc0, 0x47, 0x3c, 0xa4, 0x97, 0x79, 0xcc)]
interface IFormatProvider(IFormatProviderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetFormat(
		formatType: *mut  _Type,
		pRetVal: *mut VARIANT,
	) -> HRESULT,
}}