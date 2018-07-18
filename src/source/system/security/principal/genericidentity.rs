use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x9a37d8b2, 0x2256, 0x3fe3, 0x8b, 0xf0, 0x4f, 0xc4, 0x21, 0xa1, 0x24, 0x4f)]
interface _GenericIdentity(_GenericIdentityVtbl): IDispatch(IDispatchVtbl)  
{}} // ClaimsIdentity