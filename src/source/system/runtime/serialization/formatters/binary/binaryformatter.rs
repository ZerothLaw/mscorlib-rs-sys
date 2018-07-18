use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x3bcf0cb2, 0xa849, 0x375e, 0x81, 0x89, 0x1b, 0xa5, 0xf1, 0xf4, 0xa9, 0xb0)]
interface _BinaryFormatter(_BinaryFormatterVtbl): IDispatch(IDispatchVtbl)  
{}} //implements IRemotingFormatter/IFormatter