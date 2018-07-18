use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

use source::system::runtime::serialization::iserializationsurrogate::ISerializationSurrogate;
use source::system::runtime::serialization::streamingcontext::StreamingContext;
use source::system::reflection::cominterfaces::_Type;

RIDL!{#[uuid(0x7c66ff18, 0xa1a5, 0x3e19, 0x85, 0x7b, 0x0e, 0x7b, 0x6a, 0x9e, 0x3f, 0x38)]
interface ISurrogateSelector(ISurrogateSelectorVtbl): IDispatch(IDispatchVtbl)  
{
    fn ChainSelector(
        selector: *mut ISurrogateSelector,
    ) -> HRESULT,
    fn GetSurrogate(
        Type_: *mut _Type, 
        Context: StreamingContext, 
        selector: *mut *mut ISurrogateSelector, 
        pRetVal: *mut *mut ISerializationSurrogate,
    ) -> HRESULT,
    fn GetNextSelector(
		pRetVal: *mut *mut  ISurrogateSelector ,
	) -> HRESULT,
}}