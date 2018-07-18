use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("3eb29914-f9a9-3c15-a03f-560885cfcb61"))
ENUM!{enum RegistryPermissionAccess
{
    RegistryPermissionAccess_NoAccess = 0,
    RegistryPermissionAccess_Read = 1,
    RegistryPermissionAccess_Write = 2,
    RegistryPermissionAccess_Create = 4,
    RegistryPermissionAccess_AllAccess = 7,
}}

RIDL!{#[uuid(0xc3fb5510, 0x3454, 0x3b31, 0xb6, 0x4f, 0xde, 0x6a, 0xad, 0x6b, 0xe8, 0x20)]
interface _RegistryPermission(_RegistryPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission, IBuiltInPermission