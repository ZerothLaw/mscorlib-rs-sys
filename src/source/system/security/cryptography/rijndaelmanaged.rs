use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x427ea9d3, 0x11d8, 0x3e38, 0x9e, 0x05, 0xa4, 0xf7, 0xfa, 0x68, 0x41, 0x83)]
interface _RijndaelManaged(_RijndaelManagedVtbl): IDispatch(IDispatchVtbl)  
{}} //Rijndael
