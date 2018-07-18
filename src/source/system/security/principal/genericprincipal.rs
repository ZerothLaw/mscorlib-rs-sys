use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xb4701c26, 0x1509, 0x3726, 0xb2, 0xe1, 0x40, 0x9a, 0x63, 0x6c, 0x9b, 0x4f)]
interface _GenericPrincipal(_GenericPrincipalVtbl): IDispatch(IDispatchVtbl)  
{}} //ClaimsPrincipal