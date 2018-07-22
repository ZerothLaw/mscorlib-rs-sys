use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xec7cac31, 0x08a2, 0x393b, 0xbd, 0xf2, 0xd0, 0x52, 0xeb, 0x53, 0xaf, 0x2c)]
interface _UrlIdentityPermission(_UrlIdentityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IBuiltInPermission