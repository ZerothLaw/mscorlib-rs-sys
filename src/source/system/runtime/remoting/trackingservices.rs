use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::VARIANT;

use system::runtime::remoting::objref::_ObjRef;

RIDL!{#[uuid(0x03ec7d10, 0x17a5, 0x3585, 0x9a, 0x2e, 0x05, 0x96, 0xfc, 0xac, 0x38, 0x70)]
interface ITrackingHandler(ITrackingHandlerVtbl): IDispatch(IDispatchVtbl)  
{
    fn MarshaledObject(
        obj: VARIANT, 
        ORR: *mut _ObjRef,
    ) -> HRESULT,

    fn UnmarshaledObject(
        obj: VARIANT, 
        ORR: *mut _ObjRef,
    ) -> HRESULT,
    
    fn DisconnectedObject(
        obj: VARIANT,
    ) -> HRESULT,
}}