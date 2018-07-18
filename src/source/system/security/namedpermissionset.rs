use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xba3e053f, 0xade3, 0x3233, 0x87, 0x4a, 0x16, 0xe6, 0x24, 0xc9, 0xa4, 0x9b)]
interface _NamedPermissionSet(_NamedPermissionSetVtbl): IDispatch(IDispatchVtbl)  
{}}//PermissionSet