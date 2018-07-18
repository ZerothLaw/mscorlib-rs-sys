use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x38b2f8d7, 0x8cf4, 0x323b, 0x9c, 0x17, 0x9c, 0x55, 0xee, 0x28, 0x7a, 0x63)]
interface _ZoneIdentityPermission(_ZoneIdentityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IBuiltInPermission