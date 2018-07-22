use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("dfaecf33-4728-382d-a34d-c1b0392f8b73"))
ENUM!{enum PermissionState
{
    PermissionState_Unrestricted = 1,
    PermissionState_None = 0,
}}

RIDL!{#[uuid(0x7c6b06d1, 0x63ad, 0x35ef, 0xa9, 0x38, 0x14, 0x9b, 0x4a, 0xd9, 0xa7, 0x1f)]
interface _PrincipalPermission(_PrincipalPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} // IPermission, IUnrestrictedPermission, ISecurityEncodable, IBuiltInPermission