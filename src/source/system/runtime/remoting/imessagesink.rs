use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use system::runtime::remoting::imessage::IMessage;
use system::runtime::remoting::imessagectrl::IMessageCtrl;

RIDL!{#[uuid(0x941f8aaa, 0xa353, 0x3b1d, 0xa0, 0x19, 0x12, 0xe4, 0x43, 0x77, 0xf1, 0xcd)]
interface IMessageSink(IMessageSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn SyncProcessMessage(
		msg: *mut  IMessage ,
		pRetVal: *mut *mut  IMessage ,
	) -> HRESULT,
    fn AsyncProcessMessage(
        msg: *mut IMessage, 
        replySink: *mut IMessageSink, 
        pRetVal: *mut *mut IMessageCtrl, 
    ) -> HRESULT,
    fn get_NextSink(
		pRetVal: *mut *mut  IMessageSink ,
	) -> HRESULT,
}}