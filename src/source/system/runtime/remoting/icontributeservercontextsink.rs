use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

 use system::runtime::remoting::IMessageSink;

RIDL!{#[uuid(0x0caa23ec, 0xf78c, 0x39c9, 0x8d, 0x25, 0xb7, 0xa9, 0xce, 0x40, 0x97, 0xa7)]
interface IContributeServerContextSink(IContributeServerContextSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetServerContextSink(
		NextSink: *mut  IMessageSink ,
		pRetVal: *mut *mut  IMessageSink ,
	) -> HRESULT,
}}