use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xd94ed9bf, 0xc065, 0x3703, 0x81, 0xa2, 0x2f, 0x76, 0xea, 0x8e, 0x31, 0x2f)]
interface _Url(_UrlVtbl): IDispatch(IDispatchVtbl)  
{}} //EvidenceBase, IIdentityPermissionFactory