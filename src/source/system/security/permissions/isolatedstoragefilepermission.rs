use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x0d0c83e8, 0xbde1, 0x3ba5, 0xb1, 0xef, 0xa8, 0xfc, 0x68, 0x6d, 0x8b, 0xc9)]
interface _IsolatedStorageFilePermission(_IsolatedStorageFilePermissionVtbl): IDispatch(IDispatchVtbl)  
{}} // IsolatedStoragePermission, IBuiltInPermission
