use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::ntdef::{HRESULT};

use winapi::um::oaidl::{VARIANT};

use dispatch::ISurrogateSelector;
use dispatch::_SerializationBinder;
use dispatch::_Stream;

use source::system::runtime::serialization::streamingcontext::StreamingContext;

RIDL!{#[uuid(0x93d7a8c5, 0xd2eb, 0x319b, 0xa3, 0x74, 0xa6, 0x5d, 0x32, 0x1f, 0x2a, 0xa9)]
interface IFormatter(IFormatterVtbl): IDispatch(IDispatchVtbl)  
{
    fn Deserialize(
		serializationStream: *mut  _Stream ,
		pRetVal: *mut VARIANT ,
	) -> HRESULT,
    fn Serialize(
        serializationStream: *mut _Stream, 
        graph: VARIANT, 
    ) -> HRESULT,
    fn get_SurrogateSelector(
		pRetVal: *mut *mut  ISurrogateSelector ,
	) -> HRESULT,
    fn putref_SurrogateSelector(
        pRetVal: *mut ISurrogateSelector, 
    ) -> HRESULT,
    fn get_Binder(
		pRetVal: *mut *mut  _SerializationBinder ,
	) -> HRESULT,
    fn putref_Binder(
        pRetVal: *mut _SerializationBinder,
    ) -> HRESULT,
    fn get_Context(
        pRetVal: StreamingContext ,
    ) -> HRESULT,
    fn put_Context(
        pRetVal: StreamingContext,
    ) -> HRESULT,
}}