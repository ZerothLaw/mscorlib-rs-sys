use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xa1c392fc, 0x314c, 0x39d5, 0x8d, 0xe6, 0x1f, 0x8e, 0xbc, 0xa0, 0xa1, 0xe2)]
interface _SoapFault(_SoapFaultVtbl): IDispatch(IDispatchVtbl)  
{}} //ISerializable