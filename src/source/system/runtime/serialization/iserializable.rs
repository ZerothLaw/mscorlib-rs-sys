use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use system::runtime::serialization::serializationinfo::_SerializationInfo;
use system::runtime::serialization::streamingcontext::StreamingContext;

RIDL!{#[uuid(0xd0eeaa62, 0x3d30, 0x3ee2, 0xb8, 0x96, 0xa2, 0xf3, 0x4d, 0xda, 0x47, 0xd8)]
interface ISerializable(ISerializableVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetObjectData(
        info: *mut _SerializationInfo,
        Context: StreamingContext,
    ) -> HRESULT,
}}