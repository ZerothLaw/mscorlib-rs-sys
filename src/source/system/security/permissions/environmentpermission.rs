use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("ec73fceb-1aea-3a57-b953-21368e992507"))
ENUM!{enum EnvironmentPermissionAccess
{
    EnvironmentPermissionAccess_NoAccess = 0,
    EnvironmentPermissionAccess_Read = 1,
    EnvironmentPermissionAccess_Write = 2,
    EnvironmentPermissionAccess_AllAccess = 3,
}}

RIDL!{#[uuid(0x0720590d, 0x5218, 0x352a, 0xa3, 0x37, 0x54, 0x49, 0xe6, 0xbd, 0x19, 0xda)]
interface _EnvironmentPermission(_EnvironmentPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission, IBuiltInPermission
