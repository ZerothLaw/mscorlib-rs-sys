use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xd26a9704, 0xbf99, 0x3a3f, 0xac, 0x55, 0x96, 0xaf, 0x1a, 0x31, 0x4c, 0x7f)]
interface _ClaimsPrincipal(_ClaimsPrincipalVtbl): IDispatch(IDispatchVtbl)  
{}} //implements IPrincipal