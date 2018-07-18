use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x5f1562fb, 0x0160, 0x3655, 0xba, 0xea, 0xb1, 0x5b, 0xef, 0x60, 0x91, 0x61)]
interface _StrongNameIdentityPermission(_StrongNameIdentityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IBuiltInPermission