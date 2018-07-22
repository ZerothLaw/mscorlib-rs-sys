
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x7ae4b03c, 0x414a, 0x36e0, 0xba, 0x68, 0xf9, 0x60, 0x30, 0x04, 0xc9, 0x25)]
interface _RandomNumberGenerator(_RandomNumberGeneratorVtbl): IDispatch(IDispatchVtbl)  
{}} //abstract, IDisposable
