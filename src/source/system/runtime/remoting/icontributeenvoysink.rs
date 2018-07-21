use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use system::_MarshalByRefObject;
use system::runtime::remoting::IMessageSink;

RIDL!{#[uuid(0x124777b6, 0x0308, 0x3569, 0x97, 0xe5, 0xe6, 0xfe, 0x88, 0xea, 0xe4, 0xeb)]
interface IContributeEnvoySink(IContributeEnvoySinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetEnvoySink(
        obj: *mut _MarshalByRefObject,
        NextSink: *mut IMessageSink, 
        pRetVal: *mut *mut IMessageSink,
    ) -> HRESULT,
}}