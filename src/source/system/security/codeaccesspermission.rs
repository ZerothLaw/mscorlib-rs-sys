use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x4803ce39, 0x2f30, 0x31fc, 0xb8, 0x4b, 0x5a, 0x01, 0x41, 0x38, 0x52, 0x69)]
interface _CodeAccessPermission(_CodeAccessPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //IPermission, ISecurityEncodable, IStackWalk