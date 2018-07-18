use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use dispatch::_MarshalByRefObject;
use dispatch::IMessageSink;

RIDL!{#[uuid(0x6a5d38bc, 0x2789, 0x3546, 0x81, 0xa1, 0xf1, 0x0c, 0x0f, 0xb5, 0x93, 0x66)]
interface IContributeObjectSink(IContributeObjectSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetObjectSink(
        obj: *mut _MarshalByRefObject,
        NextSink: *mut IMessageSink, 
        pRetVal: *mut *mut IMessageSink,
    ) -> HRESULT,
}}