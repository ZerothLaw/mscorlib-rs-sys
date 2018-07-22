use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xbc0847b2, 0xbd5c, 0x37b3, 0xba, 0x67, 0x7d, 0x2d, 0x54, 0xb1, 0x72, 0x38)]
interface _SoapMessage(_SoapMessageVtbl): IDispatch(IDispatchVtbl)  
{}} //implements ISoapMessage;