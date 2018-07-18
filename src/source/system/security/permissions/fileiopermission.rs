use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("ca10c1a1-9fdc-36a3-ad74-8fac60e6541c"))
ENUM!{enum FileIOPermissionAccess
{
    FileIOPermissionAccess_NoAccess = 0,
    FileIOPermissionAccess_Read = 1,
    FileIOPermissionAccess_Write = 2,
    FileIOPermissionAccess_Append = 4,
    FileIOPermissionAccess_PathDiscovery = 8,
    FileIOPermissionAccess_AllAccess = 15,
}}

RIDL!{#[uuid(0xa2ed7efc, 0x8e59, 0x3ccc, 0xae, 0x92, 0xea, 0x23, 0x77, 0xf4, 0xd5, 0xef)]
interface _FileIOPermission(_FileIOPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}