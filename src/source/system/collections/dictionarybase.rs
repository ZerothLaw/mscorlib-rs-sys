use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xddd44da2, 0xbc6b, 0x3620, 0x93, 0x17, 0xc0, 0x37, 0x29, 0x68, 0xc7, 0x41)]
interface _DictionaryBase(_DictionaryBaseVtbl): IDispatch(IDispatchVtbl)  
{}}