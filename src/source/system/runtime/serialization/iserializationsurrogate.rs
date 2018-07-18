use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::oaidl::VARIANT;
use winapi::shared::winerror::HRESULT;

use source::system::runtime::serialization::streamingcontext::StreamingContext;
use source::system::runtime::serialization::isurrogateselector::ISurrogateSelector;
use source::system::runtime::serialization::serializationinfo::_SerializationInfo;

RIDL!{#[uuid(0x62339172, 0xdbfa, 0x337b, 0x8a, 0xc8, 0x05, 0x3b, 0x24, 0x1e, 0x06, 0xab)]
interface ISerializationSurrogate(ISerializationSurrogateVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetObjectData(
        obj: VARIANT, 
        info: *mut _SerializationInfo, 
        Context: StreamingContext,
    ) -> HRESULT,
    fn SetObjectData(
        obj: VARIANT, 
        info: *mut _SerializationInfo, 
        Context: StreamingContext, 
        selector: *mut ISurrogateSelector, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
}}