use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use system::runtime::remoting::IMessageSink;

RIDL!{#[uuid(0x4db956b7, 0x69d0, 0x312a, 0xaa, 0x75, 0x44, 0xfb, 0x55, 0xfd, 0x5d, 0x4b)]
interface IContributeClientContextSink(IContributeClientContextSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetClientContextSink(
		NextSink: *mut  IMessageSink ,
		pRetVal: *mut *mut  IMessageSink ,
	) -> HRESULT,
}}