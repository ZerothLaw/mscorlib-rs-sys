use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::SAFEARRAY;
use winapi::um::oaidl::VARIANT;

use dispatch::_Stream;
use dispatch::_HeaderHandler;

RIDL!{#[uuid(0xae1850fd, 0x3596, 0x3727, 0xa2, 0x42, 0x2f, 0xc3, 0x1c, 0x5a, 0x03, 0x12)]
interface IRemotingFormatter(IRemotingFormatterVtbl): IDispatch(IDispatchVtbl)  
{
    fn Deserialize(
        serializationStream: *mut _Stream,
        handler: *mut _HeaderHandler,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,

    fn Serialize(
        serializationStream: *mut _Stream, 
        graph: VARIANT,
        headers: *mut SAFEARRAY,
    ) -> HRESULT,
}}
