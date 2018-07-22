use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x42ca6b3f, 0x8cb9, 0x3005, 0xa7, 0xc1, 0xee, 0x90, 0x21, 0xdb, 0x36, 0x9b)]
interface _ClaimsIdentity(_ClaimsIdentityVtbl): IDispatch(IDispatchVtbl)  
{}} //implements IIdentity