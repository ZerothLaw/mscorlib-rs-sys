use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use dispatch::IDynamicMessageSink;

RIDL!{#[uuid(0xa0fe9b86, 0x0c06, 0x32ce, 0x85, 0xfa, 0x2f, 0xf1, 0xb5, 0x86, 0x97, 0xfb)]
interface IContributeDynamicSink(IContributeDynamicSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetDynamicSink(
		pRetVal: *mut *mut  IDynamicMessageSink ,
	) -> HRESULT,
}}
