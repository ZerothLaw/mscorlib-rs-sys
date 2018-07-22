use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use system::collections::IDictionary;

RIDL!{#[uuid(0x1a8b0de6, 0xb825, 0x38c5, 0xb7, 0x44, 0x8f, 0x93, 0x07, 0x5f, 0xd6, 0xfa)]
interface IMessage(IMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Properties(
		pRetVal: *mut *mut  IDictionary ,
	) -> HRESULT,
}}