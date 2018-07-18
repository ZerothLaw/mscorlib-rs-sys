use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x5f19e082, 0x26f8, 0x3361, 0xb3, 0x38, 0x9b, 0xac, 0xb9, 0x88, 0x09, 0xa4)]
interface _GacIdentityPermissionAttribute(_GacIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessSecurityAttribute

RIDL!{#[uuid(0xa9637792, 0x5be8, 0x3c93, 0xa5, 0x01, 0x49, 0xf0, 0xe8, 0x40, 0xde, 0x38)]
interface _GacIdentityPermission(_GacIdentityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IBuiltInPermission
