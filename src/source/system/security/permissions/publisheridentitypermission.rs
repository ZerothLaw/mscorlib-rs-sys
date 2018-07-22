use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x6722c730, 0x1239, 0x3784, 0xac, 0x94, 0xc2, 0x85, 0xae, 0x5b, 0x90, 0x1a)]
interface _PublisherIdentityPermissionAttribute(_PublisherIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IBuiltInPermission