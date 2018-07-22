use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("44c2f476-9e95-3d5a-b666-fdbef071494e"))
ENUM!{enum ReflectionPermissionFlag
{
    ReflectionPermissionFlag_NoFlags = 0,
    ReflectionPermissionFlag_TypeInformation = 1,
    ReflectionPermissionFlag_MemberAccess = 2,
    ReflectionPermissionFlag_ReflectionEmit = 4,
    ReflectionPermissionFlag_AllFlags = 7,
}}

RIDL!{#[uuid(0xaeb3727f, 0x5c3a, 0x34c4, 0xbf, 0x18, 0xa3, 0x8f, 0x08, 0x8a, 0xc8, 0xc7)]
interface _ReflectionPermission(_ReflectionPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission, IBuiltInPermission