use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xc2af4970, 0x4fb6, 0x319c, 0xa8, 0xaa, 0x06, 0x14, 0xd2, 0x7f, 0x2b, 0x2c)]
interface _PermissionSet(_PermissionSetVtbl): IDispatch(IDispatchVtbl)  
{}} //ISecurityEncodable, ICollection, IStackWalk, IDeserializationCallback